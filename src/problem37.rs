/// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
/// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
/// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
use crate::utils::*;
use std::collections::HashSet;

pub fn main() -> i32 {
    let primes = &get_primes_set(1_000_000)
        .iter()
        .map(|&x| x as u32)
        .collect();
    let mut left_trunc_primes: Vec<u32> = vec![];
    for i in 10..1_000_000 {
        if is_left_trunc_prime(&i, &primes) {
            left_trunc_primes.push(i);
        }
    }
    let mut res = 0;
    for p in left_trunc_primes {
        if is_right_trunc_prime(&p, &primes) {
            res += p;
        }
    }
    res as i32
}

fn is_right_trunc_prime(num: &u32, primes: &HashSet<u32>) -> bool {
    if *num == 0 {
        return true;
    }
    let trunc = num / 10;
    primes.contains(&num) && is_right_trunc_prime(&trunc, &primes)
}

fn is_left_trunc_prime(num: &u32, primes: &HashSet<u32>) -> bool {
    if *num == 0 {
        return true;
    }
    let trunc = num % 10_u32.pow((num.to_string().len() - 1) as u32);
    primes.contains(&num) && is_left_trunc_prime(&trunc, &primes)
}

#[test]
fn test() {
    let primes = &get_primes_set(1_000_000)
        .iter()
        .map(|&x| x as u32)
        .collect();
    assert_eq!(is_right_trunc_prime(&23, &primes), true);
    assert_eq!(is_right_trunc_prime(&3797, &primes), true);
    assert_eq!(is_right_trunc_prime(&19, &primes), false);
    assert_eq!(is_left_trunc_prime(&3797, &primes), true);
    assert_eq!(is_left_trunc_prime(&19, &primes), false);
}
