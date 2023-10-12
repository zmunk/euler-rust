use std::collections::HashSet;
use crate::utils::get_primes_less_than;

pub fn main() -> i64 {
    let lim = 1_000_000;
    let primes = get_primes_less_than(lim as i64);
    let primes_set = HashSet::<i64>::from_iter(primes.iter().cloned());

    let mut res = 0;
    let mut max_count = 0;
    for start_index in 0..5 {
        if start_index + max_count >= primes.len() {
            break;
        }
        let mut acc = 0;
        for &prime in primes[start_index .. start_index + max_count].iter() {
            acc += prime;
        }

        if acc > lim as i64 {
            break;
        }

        let mut count = max_count;
        for &prime in primes.iter().skip(start_index + max_count) {
            if acc > lim as i64 {
                break;
            }
            acc += prime;
            count += 1;
            if primes_set.contains(&acc) {
                max_count = count;
                res = acc;
            }
        }
    }
    res

}

#[allow(dead_code)]
fn print_sum(consecutive_primes: &Vec<i64>) {
    if consecutive_primes.len() > 0 {
        let acc = consecutive_primes.iter().fold(0, |a, b| a + b);
        let mut s = String::new();
        for c in consecutive_primes.iter() {
            s.push_str(&format!("{:?} + ", c))
        }
        println!("{} = {:?} ({:?})", s.strip_suffix(" + ").unwrap(), acc, consecutive_primes.len());
    }
}

