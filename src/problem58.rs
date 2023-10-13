use crate::utils::get_primes_set;

pub fn main() -> i32 {
    let primes_lim = 700_000_000;
    let primes = get_primes_set(primes_lim);
    println!("primes generated");
    let mut diagonal_count = 1;
    let mut prime_count = 0;
    for i in 1.. {
        for n in [
            4 * i * i - 2 * i + 1,
            4 * i * i + 1,
            4 * i * i + 2 * i + 1,
        ] {
            if n > primes_lim {
                panic!("not enough primes");
            }
            if primes.contains(&n) {
                prime_count += 1;
            }
        }
        diagonal_count += 4;
        let ratio = prime_count as f64 / diagonal_count as f64;
        if ratio < 0.1 {
            return (2 * i + 1) as i32
        }
    }
    0
}
