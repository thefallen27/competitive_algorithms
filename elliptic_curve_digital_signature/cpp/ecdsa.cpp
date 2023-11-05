#include <iostream>
#include <memory>
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/ecdsa.h>
#include <openssl/evp.h>
#include <openssl/obj_mac.h>
#include <openssl/sha.h>
#include <string>

auto BN_Deleter = [](BIGNUM* p) { BN_free(p); };
auto EC_KEY_Deleter = [](EC_KEY* p) { EC_KEY_free(p); };
auto EC_POINT_Deleter = [](EC_POINT* p) { EC_POINT_free(p); };
auto ECDSA_SIG_Deleter = [](ECDSA_SIG* p) { ECDSA_SIG_free(p); };

using BN_Ptr = std::unique_ptr<BIGNUM, decltype(BN_Deleter)>;
using EC_KEY_Ptr = std::unique_ptr<EC_KEY, decltype(EC_KEY_Deleter)>;
using EC_POINT_Ptr = std::unique_ptr<EC_POINT, decltype(EC_POINT_Deleter)>;
using ECDSA_SIG_Ptr = std::unique_ptr<ECDSA_SIG, decltype(ECDSA_SIG_Deleter)>;

void 
PrintBN(const BIGNUM* bn)
{
    char* number_string = BN_bn2hex(bn);
    std::cout << number_string << std::endl;
    OPENSSL_free(number_string);
}

int main()
{
    OpenSSL_add_all_algorithms();

    std::string message;
    std::cout << "Message to be singed: ";
    std::getline(std::cin, message);

    unsigned char hashed_value[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, message.c_str(), message.size());
    SHA256_Final(hashed_value, &sha256);

    std::cout << "Hashed value: ";
    for (auto c : hashed_value)
    {
        printf("%02x", c);
    }

    std::cout << std::endl;

    EC_KEY_Ptr elliptic_curve_key(EC_KEY_new_by_curve_name(NID_secp256k1), EC_KEY_Deleter);
    if (!elliptic_curve_key || EC_KEY_generate_key(elliptic_curve_key.get()) != 1)
    {
        return -1;
    }

    BN_Ptr k(BN_new(), BN_Deleter);
    const BIGNUM* curve_order = EC_GROUP_get0_order(EC_KEY_get0_group(elliptic_curve_key.get()));
    BN_rand_range(k.get(), curve_order);
    std::cout << "Random number k: ";
    PrintBN(k.get());

    EC_POINT_Ptr point(EC_POINT_new(EC_KEY_get0_group(elliptic_curve_key.get())), EC_POINT_Deleter);
    EC_POINT_mul(EC_KEY_get0_group(elliptic_curve_key.get()), point.get(), k.get(), NULL, NULL, NULL);
    BN_Ptr x(BN_new(), BN_Deleter);
    BN_Ptr y(BN_new(), BN_Deleter);
    EC_POINT_get_affine_coordinates_GFp(EC_KEY_get0_group(elliptic_curve_key.get()), point.get(), x.get(), y.get(), NULL);
    std::cout << "Curve point x: ";
    PrintBN(x.get());
    std::cout << "Curve point y: ";
    PrintBN(y.get());

    ECDSA_SIG_Ptr signature(ECDSA_do_sign(hashed_value, SHA256_DIGEST_LENGTH, elliptic_curve_key.get()), ECDSA_SIG_Deleter);
    if (!signature)
    {
        return -1;
    }

    const BIGNUM* sig_r, * sig_s;
    ECDSA_SIG_get0(signature.get(), &sig_r, &sig_s);
    std::cout << "Signature (r): ";
    PrintBN(sig_r);
    std::cout << "Signature (s): ";
    PrintBN(sig_s);

    int verify_status = ECDSA_do_verify(hashed_value, SHA256_DIGEST_LENGTH, signature.get(), elliptic_curve_key.get());
    if (verify_status == 1)
    {
        std::cout << "Signature is valid." << std::endl;
    }
    else
    {
        std::cout << "Signature is NOT valid." << std::endl;
    }

    return 0;
}