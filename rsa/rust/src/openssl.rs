use openssl::rsa::{Padding, Rsa};

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rsa_size = 2048;

    let rsa_key = Rsa::generate(rsa_size)?;

    println!("Enter a message to encrypt and decrypt: ");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message)?;

    let message_bytes = message.as_bytes();
    let mut encrypted_message = vec![0u8; rsa_key.size() as usize];

    let encrypted_length =
        rsa_key.public_encrypt(message_bytes, &mut encrypted_message, Padding::PKCS1)?;

    let mut decrypted_message = vec![0u8; rsa_key.size() as usize];
    let decrypted_length = rsa_key.private_decrypt(
        &encrypted_message[..encrypted_length],
        &mut decrypted_message,
        Padding::PKCS1,
    )?;

    println!(
        "Encrypted message: {:?}",
        &encrypted_message[..encrypted_length]
    );

    let mut decrypted_msg = String::new();
    for &ch in decrypted_message[..decrypted_length].iter() {
        decrypted_msg.push(char::from(ch));
    }

    println!("Decrypted message: {decrypted_msg}",);

    Ok(())
}
