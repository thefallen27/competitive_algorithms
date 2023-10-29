import random

def fast_modular_exponentiation(base, exp, mod):
    result = 1
    base = base % mod

    while exp > 0:
        if exp & 1:
            result = (result * base) % mod

        base = (base * base) % mod
        exp >>= 1

    return result

def miller_rabin(number, k=500):
    if number <= 1 or number % 2 == 0:
        return False

    if number <= 3:
        return True

    d = number - 1
    while d % 2 == 0:
        d //= 2

    for _ in range(k):
        a = random.randint(2, number - 2)
        x = fast_modular_exponentiation(a, d, number)

        if x == 1 or x == number - 1:
            continue

        prime = False
        while d != number - 1:
            x = (x * x) % number
            d *= 2

            if x == 1:
                return False

            if x == number - 1:
                prime = True
                break

        if not prime:
            return False

    return True

def find_prime_factors(number):
    factors = []

    while number % 2 == 0:
        factors.append(2)
        number //= 2

    i = 3
    while i * i <= number:
        while number % i == 0:
            if miller_rabin(i):
                factors.append(i)

            number //= i
        i += 2

    if number > 2 and miller_rabin(number):
        factors.append(number)

    return factors

if __name__ == "__main__":
    number = int(input("Enter a number: "))
    prime_factors = find_prime_factors(number)

    print("Prime factors:", " ".join(map(str, prime_factors)))
