/// Euler discovered the remarkable quadratic formula:
/// n² + n + 41
/// It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39.
/// However, when n = 40, 40² + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when
/// n = 41, 41² + 41 + 41 is clearly divisible by 41.
/// The incredible formula n² - 79n + 1601 was discovered, which produces 80 primes for the consecutive values
/// 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.
/// Considering quadratics of the form:
/// n² + an + b, where |a| < 1000 and |b| < 1000
/// where |n| is the modulus/absolute value of n
/// e.g. |11| = 11 and |-4| = 4
/// Find the product of the coefficients, a and b, for the quadratic expression
/// that produces the maximum number of primes for consecutive values of n, starting with n = 0.
use crate::utils::*;
use std::collections::HashSet;

pub fn main() -> i32 {
    let lim: i32 = 100_000;
    let primes: HashSet<i64> = HashSet::from_iter(get_primes_less_than(lim as i64).iter().cloned());

    let mut mx = 0;
    let mut prod_coef = 0;
    for a in -1000..1001 {
        for b in -1000..1001 {
            let consec = num_consec(a, b, &primes, lim);
            if consec > mx {
                mx = consec;
                prod_coef = a * b;
            }
        }
    }
    prod_coef
}

fn num_consec(a: i32, b: i32, primes: &HashSet<i64>, primes_lim: i32) -> i32 {
    let mut n = 0;
    loop {
        let v = n * n + a * n + b;
        if v > primes_lim {
            panic!("not enough primes")
        }
        if !primes.contains(&(v as i64)) {
            return n;
        }
        n += 1;
    }
}

#[test]
fn test() {
    let lim: i32 = 10_000;
    let primes: HashSet<i64> = HashSet::from_iter(get_primes_less_than(lim as i64).iter().cloned());
    assert_eq!(num_consec(1, 41, &primes, lim), 40);
    assert_eq!(num_consec(-79, 1601, &primes, lim), 80);
}
