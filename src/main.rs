mod config;
mod db;
mod server;
mod grpc;
mod firebase;
mod cert;

use config::Config;

use std::sync::Arc;

pub struct AppState {
    pub firebase: firebase::FirebaseClient,
    pub db: db::DbPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    
    println!("Starting server on {}:{}", config.host, config.port);
    
    cert::ensure_certs()?;

    let db = db::init(&config).await?;

    let firebase = firebase::FirebaseClient::new()?;
    let app_state = Arc::new(AppState { firebase, db });

    // Start HTTP/3 server
    let h3_host = config.host.clone();
    let h3_port = config.port;
    let h3_state = app_state.clone();
    tokio::spawn(async move {
        if let Err(e) = server::run(&h3_host, h3_port, h3_state).await {
            eprintln!("HTTP/3 Server error: {}", e);
        }
    });

    // Start gRPC server
    let grpc_host = config.host.clone();
    let grpc_port = config.port + 1;
    let grpc_state = app_state.clone();
    if let Err(e) = grpc::run(&grpc_host, grpc_port, grpc_state).await {
        eprintln!("gRPC Server error: {}", e);
    }

    Ok(())
}
