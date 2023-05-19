/// Returns vector of primes less than n
/// Uses sieve of eratosthenes
use std::collections::HashSet;

#[allow(dead_code)]
pub fn get_primes_less_than(n: i64) -> Vec<i64> {
    let n = (n + 1) as usize;
    let mut flags: Vec<bool> = vec![true; n]; // true -> prime
    flags[0] = false;
    flags[1] = false;

    let mut prime = 2;
    while prime * prime < n {
        let mut i = prime * prime;
        while i < n {
            flags[i] = false;
            i += prime;
        }
        prime += 1;
        while prime < n && !flags[prime] {
            prime += 1;
        }
    }

    let mut primes: Vec<i64> = vec![];
    for (i, flag) in flags.iter().enumerate() {
        if *flag {
            primes.push(i as i64)
        }
    }

    primes
}

pub fn get_primes_set(n: i64) -> HashSet<i64> {
    HashSet::from_iter(get_primes_less_than(n).iter().cloned())
}
