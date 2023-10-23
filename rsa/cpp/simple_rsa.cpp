#include <bit>
#include <cmath>
#include <concepts>
#include <iostream>
#include <numeric>
#include <random>
#include <string>
#include <tuple>

bool
IsPrime(int64_t n)
{
    if (n <= 1) return false;
    if (n <= 3) return true;

    if (n % 2 == 0 || n % 3 == 0)
    {
        return false;
    }

    for (int64_t i = 5; i * i <= n; i += 6)
    {
        if (n % i == 0 || n % (i + 2) == 0)
        {
            return false;
        }
    }

    return true;
}

int64_t
PrimeGeneration(int64_t lower, int64_t upper)
{
    std::random_device rd;
    std::mt19937_64 gen(rd());
    std::uniform_int_distribution<int64_t> distr(lower, upper);

    int64_t prime = distr(gen);
    while (!IsPrime(prime))
    {
        prime = distr(gen);
    }

    return prime;
}

auto
ExtendedGCD(int64_t a, int64_t b)
{
    int64_t old_r = a, r = b;
    int64_t old_s = 1, s = 0;
    int64_t old_t = 0, t = 1;

    while (r != 0)
    {
        int64_t quotient = old_r / r;
        std::tie(old_r, r) = std::make_tuple(r, old_r - quotient * r);
        std::tie(old_s, s) = std::make_tuple(s, old_s - quotient * s);
        std::tie(old_t, t) = std::make_tuple(t, old_t - quotient * t);
    }

    return std::make_tuple(old_r, old_s, old_t);
}

int64_t
ModularInverse(int64_t a, int64_t m)
{
    auto [g, x, y] = ExtendedGCD(a, m);
    if (g != 1)
    {
        std::cerr << "Modular inverse doesn't exist" << std::endl;
        return -1;
    }
    else
    {
        return (x % m + m) % m;
    }
}

int64_t
ModularExponentiation(int64_t base, int64_t exp, int64_t mod)
{
    int64_t result = 1;
    while (exp > 0)
    {
        if (exp & 1)
        {
            result = (result * base) % mod;
        }

        base = (base * base) % mod;
        exp >>= 1;
    }

    return result;
}

int64_t
RSAEncryption(int64_t message, int64_t e, int64_t n)
{
    return ModularExponentiation(message, e, n);
}

int64_t
RSADecryptionCRT(int64_t ciphertext, int64_t d, int64_t n, int64_t p, int64_t q)
{
    int64_t dp = d % (p - 1);
    int64_t dq = d % (q - 1);
    int64_t qinv = ModularInverse(q, p);
    int64_t m1 = ModularExponentiation(ciphertext, dp, p);
    int64_t m2 = ModularExponentiation(ciphertext, dq, q);
    int64_t h = (qinv * (m1 - m2 + p)) % p;

    return m2 + h * q;
}

int main()
{
    int64_t p = PrimeGeneration(1000, 5000);
    int64_t q = PrimeGeneration(1000, 5000);

    int64_t n = p * q;
    int64_t e = 65537;

    // Carmichael's totient function
    int64_t lambda = std::lcm(p - 1, q - 1);
    int64_t d = ModularInverse(e, lambda);

    std::cout << "p = " << p << "\nq = " << q << "\nn = " 
              << n << "\nd = " << d << "\ne = " << e << std::endl;

    std::cout << "Enter the message to encrypt: ";
    std::string msg;
    std::getline(std::cin, msg);

    std::cout << "Encrypted message: ";
    std::vector<int64_t> encrypted_characters;
    for (char c : msg)
    {
        int64_t encrypted_letter = RSAEncryption(c, e, n);
        encrypted_characters.push_back(encrypted_letter);
        std::cout << encrypted_letter << " ";
    }

    std::cout << std::endl;

    std::string decrypted_msg;
    for (int64_t encrypted_letter : encrypted_characters)
    {
        int64_t decrypted_letter = RSADecryptionCRT(encrypted_letter, d, n, p, q);
        decrypted_msg += static_cast<char>(decrypted_letter);
    }

    std::cout << "Decrypted message: " << decrypted_msg << std::endl;

    return 0;
}
