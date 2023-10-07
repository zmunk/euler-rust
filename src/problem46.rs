use std::collections::HashSet;
use crate::utils::get_primes_set;

pub fn main() -> i64 {
    let prime_lim: i64 = 10_000;
    let primes: HashSet<i64> = get_primes_set(prime_lim);

    for n in (3..).step_by(2) {
        if is_prime(&n, &primes, &prime_lim) {
            continue;
        }
        let mut i = 1;
        loop {
            let m = 2 * i * i;
            if m >= n {
                return n
            }
            if primes.contains(&(n - m).into()) {
                break;
            }
            i += 1;
        }
    }
    -1
}

fn is_prime(n: &i64, primes: &HashSet<i64>, prime_lim: &i64) -> bool {
    if n >= prime_lim {
        panic!("not enough primes");
    }
    primes.contains(n.into())
}
