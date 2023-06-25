#include <iostream>
#include <random>
#include <vector>

uint64_t 
FastModularExponentiation(uint64_t base, uint64_t exp, uint64_t mod)
{
    uint64_t result = 1;
    base = base % mod;

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

bool 
MillerRabin(uint64_t number, int k = 500)
{
    if (number <= 1 || number % 2 == 0)
    {
        return false;
    }

    if (number <= 3)
    {
        return true;
    }

    uint64_t d = number - 1;
    while (d % 2 == 0)
    {
        d /= 2;
    }

    std::random_device rd;
    std::mt19937_64 gen(rd());
    std::uniform_int_distribution<uint64_t> dis(2, number - 2);

    for (int i = 0; i < k; ++i)
    {
        uint64_t a = dis(gen);
        uint64_t x = FastModularExponentiation(a, d, number);

        if (x == 1 || x == number - 1)
        {
            continue;
        }

        bool prime = false;
        while (d != number - 1)
        {
            x = (x * x) % number;
            d *= 2;

            if (x == 1)
            {
                return false;
            }

            if (x == number - 1)
            {
                prime = true;
                break;
            }
        }

        if (!prime)
        {
            return false;
        }
    }

    return true;
}

std::vector<uint64_t> 
FindPrimeFactors(uint64_t number)
{
    std::vector<uint64_t> factors;

    while (number % 2 == 0)
    {
        factors.push_back(2);
        number /= 2;
    }

    for (uint64_t i = 3; i * i <= number; i += 2)
    {
        while (number % i == 0)
        {
            if (MillerRabin(i))
            {
                factors.push_back(i);
            }

            number /= i;
        }
    }

    if (number > 2 && MillerRabin(number))
    {
        factors.push_back(number);
    }

    return factors;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    uint64_t number;
    std::cout << "Enter a number: ";
    std::cin >> number;

    std::vector<uint64_t> prime_factors = FindPrimeFactors(number);

    std::cout << "Prime factors: ";
    for (const auto& factor : prime_factors)
    {
        std::cout << factor << " ";
    }

    std::cout << std::endl;
    return 0;
}

