/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
/// What is the largest n-digit pandigital prime that exists?
use crate::utils::*;
use std::collections::HashSet;

pub fn main() -> i64 {
    let num_dig = 7;
    let primes = get_primes_less_than(10_i64.pow(num_dig));
    println!("primes generated.");
    let mut biggest = 0;
    for p in primes.iter().rev() {
        if *p < 10_i64.pow(num_dig - 1) {
            break;
        }
        let digits: HashSet<_> = HashSet::from_iter(
            p.to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
        let mut bad = false;
        for d in 1..=num_dig {
            if !digits.contains(&d) {
                bad = true;
                break;
            }
        }
        if !bad {
            if *p > biggest {
                biggest = *p;
            }
        }
    }
    biggest
}
