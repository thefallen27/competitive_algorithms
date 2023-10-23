use std::error::Error;
use std::io;

use rand::Rng;

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

fn prime_generation(lower: i64, upper: i64) -> i64 {
    let mut rng = rand::thread_rng();
    loop {
        let prime: i64 = rng.gen_range(lower..=upper);
        if is_prime(prime) {
            return prime;
        }
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        let temp = r;
        r = old_r - quotient * r;
        old_r = temp;

        let temp = s;
        s = old_s - quotient * s;
        old_s = temp;

        let temp = t;
        t = old_t - quotient * t;
        old_t = temp;
    }

    (old_r, old_s)
}

fn modular_inverse(a: i64, m: i64) -> Result<i64, Box<dyn Error>> {
    let (g, x) = extended_gcd(a, m);
    if g != 1 {
        Err("Modular inverse doesn't exist".into())
    } else {
        let result = (x % m + m) % m;

        Ok(result)
    }
}

fn modular_exponentiation(base: i64, exp: i64, m: i64) -> i64 {
    let mut result = 1;
    let mut base = base % m;
    let mut exp = exp;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }

    result
}

fn rsa_encryption(message: i64, e: i64, n: i64) -> i64 {
    modular_exponentiation(message, e, n)
}

fn rsa_decryption_crt(ciphertext: i64, d: i64, p: i64, q: i64) -> i64 {
    let dp = d % (p - 1);
    let dq = d % (q - 1);
    let qinv = modular_inverse(q, p).unwrap();
    let m1 = modular_exponentiation(ciphertext, dp, p);
    let m2 = modular_exponentiation(ciphertext, dq, q);
    let h = (qinv * (m1 - m2 + p)) % p;
    m2 + h * q
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let p = prime_generation(1000, 5000);
    let q = prime_generation(1000, 5000);
    let n = p * q;
    let e = 65537;

    // Carmichael's totient function
    let lambda = lcm(n, lcm(p - 1, q - 1));
    let d = modular_inverse(e, lambda)?;

    println!("p = {}\nq = {}\nn = {}\nd = {}\ne = {}", p, q, n, d, e);

    let mut msg = String::new();
    println!("Enter the message to encrypt:");
    io::stdin().read_line(&mut msg)?;
    msg = msg.trim().to_string();

    println!("Encrypted message:");
    let encrypted_characters: Vec<i64> = msg
        .chars()
        .map(|c| {
            let message = c as i64;
            let encrypted_letter = rsa_encryption(message, e, n);
            print!("{} ", encrypted_letter);
            encrypted_letter
        })
        .collect();

    println!();

    print!("Decrypted message: ");
    let mut decrypted_msg = String::new();
    for encrypted_letter in &encrypted_characters {
        let decrypted_letter = rsa_decryption_crt(*encrypted_letter, d, p, q);
        print!("{} ", decrypted_letter);
        decrypted_msg.push(char::from(decrypted_letter as u8));
    }

    println!();

    println!("Decrypted text: {}", decrypted_msg);

    Ok(())
}
