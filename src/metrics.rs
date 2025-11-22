use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Simple metrics collector
#[derive(Clone)]
pub struct Metrics {
    pub total_requests: Arc<AtomicU64>,
    pub successful_requests: Arc<AtomicU64>,
    pub failed_requests: Arc<AtomicU64>,
    pub total_users_created: Arc<AtomicU64>,
    pub total_sessions_created: Arc<AtomicU64>,
    pub total_notifications_sent: Arc<AtomicU64>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            successful_requests: Arc::new(AtomicU64::new(0)),
            failed_requests: Arc::new(AtomicU64::new(0)),
            total_users_created: Arc::new(AtomicU64::new(0)),
            total_sessions_created: Arc::new(AtomicU64::new(0)),
            total_notifications_sent: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn increment_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_successful(&self) {
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failed(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_users_created(&self) {
        self.total_users_created.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_sessions_created(&self) {
        self.total_sessions_created.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_notifications_sent(&self) {
        self.total_notifications_sent.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            successful_requests: self.successful_requests.load(Ordering::Relaxed),
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
            total_users_created: self.total_users_created.load(Ordering::Relaxed),
            total_sessions_created: self.total_sessions_created.load(Ordering::Relaxed),
            total_notifications_sent: self.total_notifications_sent.load(Ordering::Relaxed),
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_users_created: u64,
    pub total_sessions_created: u64,
    pub total_notifications_sent: u64,
}

impl MetricsSnapshot {
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.successful_requests as f64 / self.total_requests as f64) * 100.0
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
  "total_requests": {},
  "successful_requests": {},
  "failed_requests": {},
  "success_rate": {:.2},
  "total_users_created": {},
  "total_sessions_created": {},
  "total_notifications_sent": {}
}}"#,
            self.total_requests,
            self.successful_requests,
            self.failed_requests,
            self.success_rate(),
            self.total_users_created,
            self.total_sessions_created,
            self.total_notifications_sent
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let metrics = Metrics::new();
        
        metrics.increment_requests();
        metrics.increment_successful();
        metrics.increment_users_created();
        
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 1);
        assert_eq!(snapshot.successful_requests, 1);
        assert_eq!(snapshot.total_users_created, 1);
        assert_eq!(snapshot.success_rate(), 100.0);
    }

    #[test]
    fn test_success_rate() {
        let metrics = Metrics::new();
        
        metrics.increment_requests();
        metrics.increment_requests();
        metrics.increment_successful();
        
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.success_rate(), 50.0);
    }
}
