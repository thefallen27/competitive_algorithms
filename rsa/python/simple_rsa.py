import random
from sympy import isprime, lcm

def prime_generation(lower, upper):
    prime = random.randint(lower, upper)
    while not isprime(prime):
        prime = random.randint(lower, upper)
    return prime

def extended_gcd(a, b):
    old_r, r = a, b
    old_s, s = 1, 0
    old_t, t = 0, 1

    while r != 0:
        quotient = old_r // r
        old_r, r = r, old_r - quotient * r
        old_s, s = s, old_s - quotient * s
        old_t, t = t, old_t - quotient * t

    return old_r, old_s, old_t

def modular_inverse(a, m):
    g, x, y = extended_gcd(a, m)
    if g != 1:
        raise Exception("Modular inverse does not exist")
    else:
        return x % m

def modular_exponentiation(base, exp, mod):
    result = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp >> 1
        base = (base * base) % mod
    return result

def rsa_encryption(message, e, n):
    return modular_exponentiation(message, e, n)

def rsa_decryption_crt(ciphertext, d, p, q):
    dp = d % (p - 1)
    dq = d % (q - 1)
    qinv = modular_inverse(q, p)
    m1 = modular_exponentiation(ciphertext, dp, p)
    m2 = modular_exponentiation(ciphertext, dq, q)
    h = (qinv * (m1 - m2 + p)) % p
    return m2 + h * q

p = prime_generation(1000, 5000000000)
q = prime_generation(1000, 5000000000)

n = p * q
e = 65537
lambda_n = lcm(p - 1, q - 1)
d = modular_inverse(e, lambda_n)

print(f"p = {p}\nq = {q}\nn = {n}\nd = {d}\ne = {e}")

msg = input("Enter the message to encrypt: ")

encrypted_message = [rsa_encryption(ord(char), e, n) for char in msg]
print("Encrypted message:", encrypted_message)

decrypted_message = "".join(chr(rsa_decryption_crt(c, d, p, q)) for c in encrypted_message)
print("Decrypted message:", decrypted_message)
