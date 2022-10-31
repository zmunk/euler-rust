use std::cmp;
use std::collections::HashMap;
use crate::utils::*; // add `mod utils` to main.rs

fn get_prime_factors(mut n: i64, primes: &Vec<i64>) -> HashMap<i64, i32> {
    let mut factors = HashMap::new();
    for p in primes {
        if n % p != 0 { continue }
        factors.insert(*p, 0);
        while n % p == 0 {
            *factors.get_mut(p).unwrap() += 1;
            n /= p;
        }
    }
    return factors;
}

pub fn main() -> i64 {
    let n: i64 = 20;
    let primes = get_primes_less_than(n);
    let mut factors: HashMap<i64, i32> = HashMap::new();
    for m in 2..=n {
        for (factor, count) in get_prime_factors(m, &primes) {
            let max_count = cmp::max(
                factors.get(&factor).unwrap_or(& 0),
                &count);
            factors.insert(factor, *max_count);
        }
    }
    let res = factors
        .iter()
        .fold(1, |a, b| a * b.0.pow(*b.1 as u32));
    return res;
}
