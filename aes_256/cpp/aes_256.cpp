#include <iostream>
#include <memory>
#include <openssl/crypto.h> 
#include <openssl/rsa.h>
#include <openssl/pem.h>
#include <openssl/err.h>
#include <string_view>
#include <vector>
#include <string>

struct BNDeleter
{
    void operator()(BIGNUM* bn) const { BN_free(bn); }
};

struct RSAKeyDeleter
{
    void operator()(RSA* rsa) const { RSA_free(rsa); }
};

struct BIODeleter
{
    void operator()(BIO* bio) const { BIO_free_all(bio); }
};

int main()
{
    using BN_ptr = std::unique_ptr<BIGNUM, BNDeleter>;
    using RSA_ptr = std::unique_ptr<RSA, RSAKeyDeleter>;
    using BIO_ptr = std::unique_ptr<BIO, BIODeleter>;

    int rsa_size = 2048;
    unsigned long e = RSA_F4;

    RSA_ptr rsa_key(RSA_new());
    BN_ptr big_number(BN_new());
    if (!BN_set_word(big_number.get(), e) || !RSA_generate_key_ex(rsa_key.get(), rsa_size, big_number.get(), nullptr))
    {
        std::cerr << "Error generating RSA key." << std::endl;
        return 1;
    }

    std::string message;
    std::cout << "Enter a message to encrypt and decrypt: ";
    std::getline(std::cin, message);

    std::vector<unsigned char> encrypted_message(RSA_size(rsa_key.get()));
    int encrypted_length = RSA_public_encrypt(message.size(), reinterpret_cast<const unsigned char*>(message.data()),
        encrypted_message.data(), rsa_key.get(), RSA_PKCS1_PADDING);
    if (encrypted_length == -1)
    {
        std::cerr << "Error encrypting message: " << ERR_error_string(ERR_get_error(), nullptr) << std::endl;
        return 1;
    }

    OPENSSL_cleanse(&message[0], message.size());

    std::vector<unsigned char> decrypted_message(RSA_size(rsa_key.get()));
    int decrypted_length = RSA_private_decrypt(encrypted_length, encrypted_message.data(),
        decrypted_message.data(), rsa_key.get(), RSA_PKCS1_PADDING);

    if (decrypted_length == -1)
    {
        OPENSSL_cleanse(encrypted_message.data(), encrypted_length);
        std::cerr << "Error decrypting message: " << ERR_error_string(ERR_get_error(), nullptr) << std::endl;
        return 1;
    }

    std::cout << "Encrypted message: ";
    for (int i = 0; i < encrypted_length; ++i)
    {
        printf("%02x", encrypted_message[i]);
    }

    std::cout << std::endl << "Decrypted message: "
        << std::string_view(reinterpret_cast<char*>(decrypted_message.data()), decrypted_length)
        << std::endl;

    OPENSSL_cleanse(decrypted_message.data(), decrypted_length);

    return 0;
}
