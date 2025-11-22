use rcgen::generate_simple_self_signed;
use std::fs;
use std::path::Path;

pub fn ensure_certs() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new("cert.crt").exists() && Path::new("cert.key").exists() {
        println!("Certificates already exist.");
        return Ok(());
    }

    println!("Generating self-signed certificates...");
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names)?;
    
    fs::write("cert.crt", cert.serialize_pem()?)?;
    fs::write("cert.key", cert.serialize_private_key_pem())?;

    println!("Certificates generated successfully.");
    Ok(())
}
