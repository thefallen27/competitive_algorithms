use rand::Rng;

fn fast_modular_exponentiation(base: u64, exp: u64, modulo: u64) -> u64 {
    let mut result = 1;
    let mut b = base % modulo;
    let mut exp = exp;

    while exp > 0 {
        if (exp & 1) > 0 {
            result = (result * b) % modulo;
        }

        b = (b * b) % modulo;
        exp >>= 1;
    }

    result
}

fn miller_rabin(number: u64, k: u32) -> bool {
    if number <= 1 || number % 2 == 0 {
        return false;
    }

    if number <= 3 {
        return true;
    }

    let mut d = number - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a = rng.gen_range(2..number - 2);
        let mut x = fast_modular_exponentiation(a, d, number);

        if x == 1 || x == number - 1 {
            continue;
        }

        let mut prime = false;
        while d != number - 1 {
            x = (x * x) % number;
            d *= 2;

            if x == 1 {
                return false;
            }

            if x == number - 1 {
                prime = true;
                break;
            }
        }

        if !prime {
            return false;
        }
    }

    true
}

fn find_prime_factors(number: u64, k: u32) -> Vec<u64> {
    let mut factors = vec![];
    let mut number = number;

    while number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }

    let mut i = 3;
    while i <= number {
        while number % i == 0 {
            if miller_rabin(i, k) {
                factors.push(i);
            }

            number /= i;
        }

        i += 2;
    }

    if number > 2 && miller_rabin(number, k) {
        factors.push(number);
    }

    factors
}

fn main() {
    let numbers = vec![8, 1288889821781, 340332181802372051];

    for num in numbers {
        let prime_factors = find_prime_factors(num, 500);

        println!("Number: {num}");
        println!("Prime factors: {prime_factors:?}");
    }
}
