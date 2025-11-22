mod config;
mod db;
mod server;
mod grpc;
mod firebase;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    
    println!("Starting server on {}:{}", config.host, config.port);
    
    db::init().await;

    let _firebase = firebase::FirebaseClient::new()?;

    // Start HTTP/3 server
    let h3_host = config.host.clone();
    let h3_port = config.port;
    tokio::spawn(async move {
        if let Err(e) = server::run(&h3_host, h3_port).await {
            eprintln!("HTTP/3 Server error: {}", e);
        }
    });

    // Start gRPC server
    let grpc_host = config.host.clone();
    let grpc_port = config.port + 1;
    if let Err(e) = grpc::run(&grpc_host, grpc_port).await {
        eprintln!("gRPC Server error: {}", e);
    }

    Ok(())
}
