use quinn::{ClientConfig, Endpoint};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, SignatureScheme};
use std::sync::Arc;

#[derive(Debug)]
struct SkipServerVerification;

impl ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ED25519,
        ]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkWithMentor HTTP/3 Client - Comprehensive Testing\n");
    println!("=" .repeat(60));

    // Create TLS config
    let mut crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    crypto.alpn_protocols = vec![b"h3".to_vec()];

    let client_config = ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?
    ));

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    // Test 1: Basic connection
    println!("\nTest 1: Basic Connection");
    println!("=" .repeat(60));
    test_basic_connection(&endpoint).await?;

    // Test 2: Multiple streams
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 2: Multiple Bidirectional Streams");
    println!("=" .repeat(60));
    test_multiple_streams(&endpoint).await?;

    // Test 3: Large message
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 3: Large Message Transfer");
    println!("=" .repeat(60));
    test_large_message(&endpoint).await?;

    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("✅ All HTTP/3 tests completed successfully!");
    println!("=" .repeat(60));

    endpoint.wait_idle().await;
    Ok(())
}

async fn test_basic_connection(endpoint: &Endpoint) -> Result<(), Box<dyn std::error::Error>> {
    println!("  → Connecting to 127.0.0.1:3000...");
    let connection = endpoint.connect("127.0.0.1:3000".parse()?, "localhost")?.await?;
    println!("  → Connected! Opening stream...");

    let (mut send, mut recv) = connection.open_bi().await?;

    let message = b"Hello from HTTP/3 test client!";
    send.write_all(message).await?;
    send.finish()?;
    println!("  → Sent: {}", String::from_utf8_lossy(message));

    let response = recv.read_to_end(1024).await?;
    println!("  → Received: {}", String::from_utf8_lossy(&response));

    connection.close(0u32.into(), b"test complete");
    println!("✅ Basic connection test passed");

    Ok(())
}

async fn test_multiple_streams(endpoint: &Endpoint) -> Result<(), Box<dyn std::error::Error>> {
    println!("  → Connecting to 127.0.0.1:3000...");
    let connection = endpoint.connect("127.0.0.1:3000".parse()?, "localhost")?.await?;
    println!("  → Opening 5 concurrent streams...");

    let mut handles = vec![];

    for i in 1..=5 {
        let conn = connection.clone();
        let handle = tokio::spawn(async move {
            let (mut send, mut recv) = conn.open_bi().await?;
            
            let message = format!("Stream {} message", i);
            send.write_all(message.as_bytes()).await?;
            send.finish()?;

            let response = recv.read_to_end(1024).await?;
            println!("  → Stream {}: {}", i, String::from_utf8_lossy(&response));

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    connection.close(0u32.into(), b"test complete");
    println!("✅ Multiple streams test passed");

    Ok(())
}

async fn test_large_message(endpoint: &Endpoint) -> Result<(), Box<dyn std::error::Error>> {
    println!("  → Connecting to 127.0.0.1:3000...");
    let connection = endpoint.connect("127.0.0.1:3000".parse()?, "localhost")?.await?;
    println!("  → Opening stream for large message...");

    let (mut send, mut recv) = connection.open_bi().await?;

    // Send 10KB message
    let large_message = vec![b'A'; 10 * 1024];
    send.write_all(&large_message).await?;
    send.finish()?;
    println!("  → Sent: {} bytes", large_message.len());

    let response = recv.read_to_end(100 * 1024).await?;
    println!("  → Received: {} bytes", response.len());

    connection.close(0u32.into(), b"test complete");
    println!("✅ Large message test passed");

    Ok(())
}
