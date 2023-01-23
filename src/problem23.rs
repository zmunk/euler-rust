/// A perfect number is a number for which the sum of its proper divisors is
/// exactly equal to the number. For example, the sum of the proper divisors of
/// 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect
/// number.
/// A number n is called deficient if the sum of its proper divisors is less
/// than n and it is called abundant if this sum exceeds n.
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest
/// number that can be written as the sum of two abundant numbers is 24. By
/// mathematical analysis, it can be shown that all integers greater than 28123
/// can be written as the sum of two abundant numbers. However, this upper
/// limit cannot be reduced any further by analysis even though it is known
/// that the greatest number that cannot be expressed as the sum of two
/// abundant numbers is less than this limit.
/// Find the sum of all the positive integers which cannot be written as the
/// sum of two abundant numbers.
use crate::utils::*;
use std::collections::HashMap;

pub fn main() -> i32 {
    println!("Generating primes...");
    let primes: Vec<i64> = get_primes_less_than(100_000);
    println!("Primes generated successfully!");

    let upper_limit: i32 = 28123;
    let mut flags: Vec<bool> = vec![false; upper_limit as usize]; // false -> unmarked

    let mut abundant_numbers: Vec<i32> = Vec::new();
    for i in 1i32..=upper_limit {
        if is_abundant(&primes, i) {
            abundant_numbers.push(i);
            for j in &abundant_numbers {
                let u = (i + j) as usize;
                if u < flags.len() {
                    flags[u] = true;
                } else {
                    break;
                }
            }
        }
    }
    let mut res: i32 = 0;
    for (i, f) in flags.iter().enumerate() {
        if !f {
            res += i as i32;
            println!("{:?}", i);
        }
    }
    return res;
}

fn is_abundant(primes: &Vec<i64>, v: i32) -> bool {
    let factors = get_prime_factors(&primes, &v);
    let sum = get_sum_of_divisors(&factors, &v);
    return sum > v;
}

fn get_sum_of_divisors(factors: &HashMap<i32, i32>, v: &i32) -> i32 {
    let mut divisors: Vec<i32> = vec![1];
    let mut sum = 1;
    for (k, v) in factors {
        for e in divisors.clone() {
            let mut val = 1;
            for _ in 0..*v {
                val *= k;
                let n = e * val;
                divisors.push(n);
                sum += n;
            }
        }
    }
    return sum - v;
}

fn get_prime_factors(primes: &Vec<i64>, v: &i32) -> HashMap<i32, i32> {
    let mut v: i32 = *v;
    let mut factors: HashMap<i32, i32> = HashMap::new();
    if v == 1 {
        return factors;
    }
    for p in primes.iter().map(|p| *p as i32) {
        if v % p != 0 {
            continue;
        }
        let mut c = 0;
        while v % p == 0 {
            c += 1;
            v /= p;
        }
        factors.insert(p, c);
        if v == 1 {
            return factors;
        }
    }
    panic!("not enough primes generated");
}
