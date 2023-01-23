/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n
/// which divide evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair
/// and each of a and b are called amicable numbers.
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44,
/// 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4,
/// 71 and 142; so d(284) = 220.
/// Evaluate the sum of all the amicable numbers under 10000.
use crate::utils::*;
use std::collections::HashMap;

pub fn main() -> i32 {
    println!("Generating primes...");
    let primes: Vec<i64> = get_primes_less_than(10_000_000);
    let mut sums: HashMap<i32, i32> = HashMap::new();
    println!("Primes generated successfully!");
    let mut res = 0;
    for v in 1..10_000 {
        let factors = get_prime_factors(&primes, &v);
        let sum = get_sum_of_divisors(&factors, &v);
        sums.insert(v, sum);
        if sums.contains_key(&sum) && sums[&sum] == v && sum != v {
            res += sum + v;
        }
    }
    return res;
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
