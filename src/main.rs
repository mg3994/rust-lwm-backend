mod config;
mod db;
mod server;
mod grpc;
mod firebase;
mod cert;
mod models;
mod auth;
mod rate_limit;
mod metrics;
mod health;

use config::Config;

use std::sync::Arc;

pub struct AppState {
    pub firebase: firebase::FirebaseClient,
    pub db: db::DbPool,
    pub rate_limiter: rate_limit::RateLimiter,
    pub metrics: metrics::Metrics,
    pub start_time: std::time::Instant,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,backend=debug".into())
        )
        .init();

    tracing::info!("Starting LinkWithMentor Backend Server");

    let config = Config::from_env()?;
    
    tracing::info!("Server configuration loaded: {}:{}", config.host, config.port);
    
    cert::ensure_certs()?;

    let db = db::init(&config).await?;

    let firebase = firebase::FirebaseClient::new()?;
    
    // Create rate limiter: 100 requests per minute per user
    let rate_limiter = rate_limit::RateLimiter::new(
        100,
        std::time::Duration::from_secs(60)
    );
    
    // Create metrics collector
    let metrics = metrics::Metrics::new();
    let start_time = std::time::Instant::now();
    
    let app_state = Arc::new(AppState { 
        firebase, 
        db,
        rate_limiter,
        metrics,
        start_time,
    });

    // Spawn metrics reporter (every 60 seconds)
    let metrics_state = app_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            let snapshot = metrics_state.metrics.get_snapshot();
            tracing::info!(
                "Metrics: {} total requests, {:.2}% success rate, {} users created",
                snapshot.total_requests,
                snapshot.success_rate(),
                snapshot.total_users_created
            );
        }
    });

    // Start HTTP/3 server
    let h3_host = config.host.clone();
    let h3_port = config.port;
    let h3_state = app_state.clone();
    tokio::spawn(async move {
        tracing::info!("Starting HTTP/3 server on {}:{}", h3_host, h3_port);
        if let Err(e) = server::run(&h3_host, h3_port, h3_state).await {
            tracing::error!("HTTP/3 Server error: {}", e);
        }
    });

    // Start gRPC server
    let grpc_host = config.host.clone();
    let grpc_port = config.port + 1;
    let grpc_state = app_state.clone();
    
    tracing::info!("Starting gRPC server on {}:{}", grpc_host, grpc_port);
    if let Err(e) = grpc::run(&grpc_host, grpc_port, grpc_state).await {
        tracing::error!("gRPC Server error: {}", e);
    }

    Ok(())
}
