#include <iomanip>
#include <iostream>
#include <openssl/aes.h>
#include <openssl/rand.h>
#include <sstream>
#include <string>
#include <vector>
 
std::vector<unsigned char> 
AESEncrypt(const AES_KEY& encrypt_key, const std::vector<unsigned char>& iv, const std::vector<unsigned char>& plaintext)
{
    const size_t block_size = 16;
    size_t plaintext_length = plaintext.size();
    size_t number_of_blocks = (plaintext_length + block_size - 1) / block_size;
    std::vector<unsigned char> ciphertext(number_of_blocks * block_size);
    std::vector<unsigned char> iv_copy(iv);
 
    AES_cbc_encrypt(plaintext.data(), ciphertext.data(), plaintext_length, &encrypt_key, iv_copy.data(), AES_ENCRYPT);
 
    return ciphertext;
}
 
std::vector<unsigned char>
AESDecrypt(const AES_KEY& decrypt_key, const std::vector<unsigned char>& iv, const std::vector<unsigned char>& ciphertext)
{
    std::vector<unsigned char> plaintext(ciphertext.size());
    std::vector<unsigned char> iv_copy(iv);
 
    AES_cbc_encrypt(ciphertext.data(), plaintext.data(), plaintext.size(), &decrypt_key, iv_copy.data(), AES_DECRYPT);
 
    return plaintext;
}
 
std::string
ToHex(const std::vector<unsigned char>& data)
{
    std::stringstream ss;
    ss << std::hex << std::uppercase << std::setfill('0');
    for (const auto& byte : data)
	{
        ss << std::setw(2) << static_cast<int>(byte);
    }
 
    return ss.str();
}
 
int main()
{
    std::string input_message;
    std::cout << "Enter the text to encrypt: ";
    std::getline(std::cin, input_message);

    std::vector<unsigned char> plaintext(input_message.begin(), input_message.end());
    std::vector<unsigned char> key(32); // 256 bits for AES-256 random key
    std::vector<unsigned char> iv(AES_BLOCK_SIZE);
 
    if (!RAND_bytes(key.data(), key.size()) || !RAND_bytes(iv.data(), iv.size()))
	{
        std::cerr << "Error generating random key or initialization vector." << std::endl;
        return 1;
    }
 
    AES_KEY encrypt_key, decrypt_key;
    if (AES_set_encrypt_key(key.data(), 256, &encrypt_key) < 0
	 || AES_set_decrypt_key(key.data(), 256, &decrypt_key) < 0)
	{
        std::cerr << "Error setting encryption or decryption key." << std::endl;
        return 1;
    }
 
    std::vector<unsigned char> ciphertext = AESEncrypt(encrypt_key, iv, plaintext);
    std::string hex_ciphertext = ToHex(ciphertext);
    std::cout << "Ciphertext: " << hex_ciphertext << std::endl;
 
    std::vector<unsigned char> decrypted_text = AESDecrypt(decrypt_key, iv, ciphertext);
    std::string output_text(decrypted_text.begin(), decrypted_text.end());
    std::cout << "Decrypted text: " << output_text << std::endl;
 
    OPENSSL_cleanse(key.data(), key.size());
 
    return 0;
}