/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
/// that the 6th prime is 13.
/// What is the 10 001st prime number?

use crate::utils::*; // add `mod utils` to main.rs

pub fn main() -> i64 {
    let n: i32 = 10_001;
    let mut lim: i64 = 10 * n as i64;
    let mut primes: Vec<i64>;
    primes = get_primes_less_than(lim);
    while (primes.len() as i32) < n {
        lim = 2 * lim;
        primes = get_primes_less_than(lim);
    }
    return primes[(n - 1) as usize];
}
