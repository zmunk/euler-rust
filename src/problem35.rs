/// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
/// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
/// How many circular primes are there below one million?
use crate::utils::*;

pub fn main() -> i32 {
    let primes = get_primes_set(1_000_000);
    println!("generated primes");

    let mut count = 0;
    for a in 1..1_000_000 {
        if a != 2 && contains_even_digit(a) {
            continue;
        }
        let mut is_circular = true;
        for rot in rotations(a) {
            if !primes.contains(&(rot as i64)) {
                is_circular = false;
                break;
            }
        }
        if is_circular {
            count += 1;
        }
    }
    count
}

fn contains_even_digit(mut num: u32) -> bool {
    while num > 0 {
        if (num % 10) % 2 == 0 {
            return true;
        }
        num /= 10;
    }
    false
}

/// Returns all rotated versions of num
fn rotations(mut num: u32) -> Vec<u32> {
    let mut res = vec![];
    for _ in 0..num.to_string().len() {
        res.push(num);
        num = rotate(num);
    }
    res
}

/// 197 -> 971
fn rotate(num: u32) -> u32 {
    let order = 10_u32.pow((num.to_string().len() - 1) as u32);
    (num % order) * 10 + num / order
}

#[test]
fn test() {
    assert_eq!(contains_even_digit(197), false);
    assert_eq!(contains_even_digit(107), true);
}
