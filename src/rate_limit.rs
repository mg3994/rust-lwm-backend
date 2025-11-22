use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Simple in-memory rate limiter
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    /// Check if a request from the given identifier is allowed
    pub fn check_rate_limit(&self, identifier: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        let user_requests = requests.entry(identifier.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside the window
        user_requests.retain(|&time| now.duration_since(time) < self.window);
        
        // Check if under limit
        if user_requests.len() < self.max_requests {
            user_requests.push(now);
            true
        } else {
            false
        }
    }

    /// Get remaining requests for an identifier
    pub fn remaining(&self, identifier: &str) -> usize {
        let requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        if let Some(user_requests) = requests.get(identifier) {
            let valid_requests = user_requests
                .iter()
                .filter(|&&time| now.duration_since(time) < self.window)
                .count();
            
            self.max_requests.saturating_sub(valid_requests)
        } else {
            self.max_requests
        }
    }

    /// Clean up old entries (should be called periodically)
    pub fn cleanup(&self) {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        requests.retain(|_, times| {
            times.retain(|&time| now.duration_since(time) < self.window);
            !times.is_empty()
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, Duration::from_secs(1));
        
        assert!(limiter.check_rate_limit("user1"));
        assert!(limiter.check_rate_limit("user1"));
        assert!(limiter.check_rate_limit("user1"));
        assert!(!limiter.check_rate_limit("user1")); // Should be blocked
        
        assert_eq!(limiter.remaining("user1"), 0);
    }

    #[test]
    fn test_different_users() {
        let limiter = RateLimiter::new(2, Duration::from_secs(1));
        
        assert!(limiter.check_rate_limit("user1"));
        assert!(limiter.check_rate_limit("user2"));
        assert!(limiter.check_rate_limit("user1"));
        assert!(limiter.check_rate_limit("user2"));
        
        // Both should be at limit
        assert!(!limiter.check_rate_limit("user1"));
        assert!(!limiter.check_rate_limit("user2"));
    }
}
