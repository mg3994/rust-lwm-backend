use tonic::transport::Channel;
use quinn::{ClientConfig, Endpoint};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, SignatureScheme};
use std::sync::Arc;

pub mod pb {
    tonic::include_proto!("service");
}

use pb::link_with_mentor_client::LinkWithMentorClient;
use pb::{PingRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkWithMentor Client - Testing Tool\n");
    println!("=" .repeat(50));

    // Test HTTP/3 (QUIC) connection
    println!("\n📡 Testing HTTP/3 (QUIC) connection...");
    match test_http3().await {
        Ok(_) => println!("✅ HTTP/3 test passed!"),
        Err(e) => println!("❌ HTTP/3 test failed: {}", e),
    }

    // Test gRPC connection
    println!("\n📡 Testing gRPC connection...");
    match test_grpc().await {
        Ok(_) => println!("✅ gRPC test passed!"),
        Err(e) => println!("❌ gRPC test failed: {}", e),
    }

    println!("\n" .repeat(1) + &"=".repeat(50));
    println!("✅ All tests completed!");
    Ok(())
}

// Custom certificate verifier that accepts self-signed certificates
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

async fn test_http3() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom TLS config that accepts self-signed certificates
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

    println!("  → Connecting to 127.0.0.1:3000...");
    let connection = endpoint.connect("127.0.0.1:3000".parse()?, "localhost")?.await?;
    println!("  → Connected! Opening bidirectional stream...");

    let (mut send, mut recv) = connection.open_bi().await?;

    // Send a simple message
    let message = b"Hello from HTTP/3 client!";
    send.write_all(message).await?;
    send.finish()?;
    println!("  → Sent: {}", String::from_utf8_lossy(message));

    // Receive response
    let response = recv.read_to_end(1024).await?;
    println!("  → Received: {}", String::from_utf8_lossy(&response));

    connection.close(0u32.into(), b"done");
    endpoint.wait_idle().await;

    Ok(())
}

async fn test_grpc() -> Result<(), Box<dyn std::error::Error>> {
    println!("  → Connecting to http://127.0.0.1:3001...");
    let mut client = LinkWithMentorClient::connect("http://127.0.0.1:3001").await?;

    let request = tonic::Request::new(PingRequest {
        message: "Hello from gRPC client!".into(),
    });

    println!("  → Sending Ping request...");
    let response = client.ping(request).await?;
    println!("  → Response: {}", response.into_inner().message);

    Ok(())
}
