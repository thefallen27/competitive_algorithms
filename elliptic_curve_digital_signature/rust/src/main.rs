use openssl::bn::BigNumContext;
use openssl::ec::{EcGroup, EcKey};
use openssl::ecdsa::EcdsaSig;
use openssl::sha::Sha256;
use std::io::{self, Write};

fn to_hex(data: &[u8]) -> String {
    data.iter().map(|byte| format!("{:02X}", byte)).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut message = String::new();
    print!("Message to be signed: ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut message)?;
    message = message.trim().to_string();

    if message.is_empty() {
        eprintln!("Message cannot be empty.");
        return Ok(());
    }

    let mut sha256 = Sha256::new();
    sha256.update(message.as_bytes());
    let hashed_value = sha256.finish();

    print!("Hashed value: ");
    for c in &hashed_value {
        print!("{:02x}", c);
    }
    println!();

    let ec_group = EcGroup::from_curve_name(openssl::nid::Nid::SECP256K1)?;
    let elliptic_curve_key = EcKey::generate(&ec_group)?;
    let k = elliptic_curve_key.private_key();

    println!("Random number k: {}", to_hex(k.to_vec().as_slice()));

    let point = elliptic_curve_key.public_key();
    let mut ctx = BigNumContext::new()?;
    let key_bytes = point.to_bytes(
        &ec_group,
        openssl::ec::PointConversionForm::UNCOMPRESSED,
        &mut ctx,
    )?;

    // The key_bytes will contain the x and y coordinates as follows:
    // - The first byte (0x04) indicates an uncompressed point.
    // - The next 32 bytes are the x-coordinate.
    // - The last 32 bytes are the y-coordinate.

    assert_eq!(key_bytes[0], 0x04);
    let x_coord = &key_bytes[1..33];
    let y_coord = &key_bytes[33..];

    println!("Curve point x: {}", to_hex(x_coord));
    println!("Curve point y: {}", to_hex(y_coord));

    let signature = EcdsaSig::sign(&hashed_value, &elliptic_curve_key)?;
    let sig_r = signature.r();
    let sig_s = signature.s();
    println!("Signature (r): {}", to_hex(sig_r.to_vec().as_slice()));
    println!("Signature (s): {}", to_hex(sig_s.to_vec().as_slice()));

    let verify_status = signature.verify(&hashed_value, &elliptic_curve_key)?;
    if verify_status {
        println!("Signature is valid.");
    } else {
        println!("Signature is NOT valid.");
    }

    Ok(())
}
