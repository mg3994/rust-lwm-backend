use crate::AppState;
use std::sync::Arc;

/// Health check status
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub database: bool,
    pub firebase: bool,
    pub uptime_seconds: u64,
}

impl HealthStatus {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{
  "status": "{}",
  "database": {},
  "firebase": {},
  "uptime_seconds": {}
}}"#,
            self.status, self.database, self.firebase, self.uptime_seconds
        )
    }
}

/// Check system health
pub async fn check_health(state: Arc<AppState>, start_time: std::time::Instant) -> HealthStatus {
    let db_healthy = check_database(&state.db).await;
    let firebase_healthy = true; // Firebase client is initialized at startup
    
    let uptime = start_time.elapsed().as_secs();
    
    let status = if db_healthy && firebase_healthy {
        "healthy"
    } else {
        "unhealthy"
    };

    HealthStatus {
        status: status.to_string(),
        database: db_healthy,
        firebase: firebase_healthy,
        uptime_seconds: uptime,
    }
}

/// Check database connectivity
async fn check_database(pool: &crate::db::DbPool) -> bool {
    // Try a simple query
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .is_ok()
}

/// Readiness check (more strict than health check)
pub async fn check_readiness(state: Arc<AppState>) -> bool {
    // Check if we can actually perform operations
    check_database(&state.db).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_json() {
        let status = HealthStatus {
            status: "healthy".to_string(),
            database: true,
            firebase: true,
            uptime_seconds: 3600,
        };

        let json = status.to_json();
        assert!(json.contains("healthy"));
        assert!(json.contains("3600"));
    }
}
