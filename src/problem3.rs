use crate::utils::*; // add `mod utils` to main.rs

#[allow(dead_code)]
pub fn main() -> i64 {
    let mut n: i64 = 600851475143;
    let primes = get_primes_less_than((n as f64).sqrt().ceil() as i64);

    let mut largest_prime = 0;
    loop {
        let prime = get_smallest_prime_factor(n, &primes);
        if prime > largest_prime { largest_prime = prime }
        if prime == n { break }
        n = n / prime;
    }

    return largest_prime;
}

/// Iterates through all primes from 2 to the square root of n
/// until it finds a prime that factors into n
/// then it returns this prime.
/// Returns n if no prime found that factors into n.
///
/// # Arguments
/// * n - the value whose prime factor we are looking for
/// * primes - a vector containing prime numbers
#[allow(dead_code)]
fn get_smallest_prime_factor(n: i64, primes: &Vec<i64>) -> i64 {
    for p in primes {
        if p*p > n { break }
        if n % p == 0 { return *p }
    }
    n // if no primes divide it, n is prime
}
