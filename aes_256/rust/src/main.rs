use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let input_message = read_user_input("Enter the text to encrypt: ")?;
    let plaintext: Vec<u8> = input_message.bytes().collect();

    let key = generate_aes_key();
    let iv = generate_aes_iv();

    let ciphertext = aes_encrypt(&key, &iv, &plaintext)?;
    let hex_ciphertext = to_hex(&ciphertext);
    println!("Ciphertext: {}", hex_ciphertext);

    let decrypted_text = aes_decrypt(&key, &iv, &ciphertext)?;
    let output_text =
        String::from_utf8(decrypted_text).expect("Failed to convert vector of bytes to string");
    println!("Decrypted text: {}", output_text);

    Ok(())
}

fn read_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn generate_aes_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    rand_bytes(&mut key).expect("Error generating random key");
    key
}

fn generate_aes_iv() -> Vec<u8> {
    let mut iv = vec![0u8; 16];
    rand_bytes(&mut iv).expect("Error generating initialization vector");
    iv
}

fn aes_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> io::Result<Vec<u8>> {
    let cipher = Cipher::aes_256_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;
    encrypter.pad(true);

    let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];
    let count = encrypter.update(plaintext, &mut ciphertext)?;

    let final_count = encrypter.finalize(&mut ciphertext[count..])?;
    Ok(ciphertext[..count + final_count].to_vec())
}

fn aes_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> io::Result<Vec<u8>> {
    let cipher = Cipher::aes_256_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;
    decrypter.pad(true);

    let mut plaintext = vec![0; ciphertext.len() + cipher.block_size()];
    let count = decrypter.update(ciphertext, &mut plaintext)?;

    let final_count = decrypter.finalize(&mut plaintext[count..])?;
    Ok(plaintext[..count + final_count].to_vec())
}

fn to_hex(data: &[u8]) -> String {
    data.iter().map(|byte| format!("{:02X}", byte)).collect()
}
