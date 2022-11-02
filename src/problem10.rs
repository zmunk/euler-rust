/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
/// Find the sum of all the primes below two million.

use crate::utils::*;

pub fn main() -> i64 {
    let primes: Vec<i64> = get_primes_less_than(2_000_000);
    return primes.iter().fold(0, |a, b| a as i64 + b);
}
