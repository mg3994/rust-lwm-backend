use tokio::net::UdpSocket;
use tokio_quiche::QuicListener;
use quiche::Config;
use std::sync::Arc;
use crate::AppState;

pub async fn run(host: &str, port: u16, _state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);
    let socket = UdpSocket::bind(&addr).await?;
    println!("HTTP/3 server listening on {}", addr);

    let mut config = Config::new(quiche::PROTOCOL_VERSION)?;
    config.set_application_protos(&[b"h3"])?;
    config.set_max_idle_timeout(5000);
    config.set_max_recv_udp_payload_size(1350);
    config.set_initial_max_data(10_000_000);
    config.set_initial_max_stream_data_bidi_local(1_000_000);
    config.set_initial_max_stream_data_bidi_remote(1_000_000);
    config.set_initial_max_streams_bidi(100);
    config.set_initial_max_streams_uni(100);
    config.set_disable_active_migration(true);

    config.load_cert_chain_from_pem_file("cert.crt")?;
    config.load_priv_key_from_pem_file("cert.key")?;

    // Placeholder for listener loop
    // let listener = QuicListener::new(socket, config);
    
    // loop {
    //     let connection = listener.accept().await?;
    //     tokio::spawn(async move {
    //         // Handle connection
    //     });
    // }

    println!("HTTP/3 server setup complete (SSL loaded)");
    Ok(())
}
