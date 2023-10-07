use std::collections::HashMap;
use crate::utils::get_primes_set;

fn add_to_collection(prime: i32, collection: &mut HashMap<i32, Vec<i32>>) {
    let key = simplify(prime);
    let group = collection.entry(key).or_insert(vec![]);

    // insert into group while preserving ascending order
    let (mut left, mut right) = (0, group.len());
    let mut mid;
    loop {
        mid = (left + right) / 2;
        if left == right {
            break;
        }
        if prime < group[mid] {
            right = mid;
        } else if prime > group[mid] {
            left = mid + 1;
        } else {
            break;
        }
    }
    if mid == group.len() {
        group.push(prime);
    } else {
        group.insert(mid, prime);
    }
} 

pub fn main() -> i64 {
    let primes = get_primes_set(10000);
    let mut collection: HashMap<i32, Vec<i32>> = HashMap::new();

    for i in 1_000..10_000 {
        if primes.contains(&i) {
            add_to_collection(i as i32, &mut collection);
        }
    }

    for (key, group) in collection {
        if key == 1478 {
            continue;
        } 
        let mut diffs = HashMap::new();
        for i in 0..group.len() {
            for j in i+1..group.len() {
                let diff = group[j] - group[i];
                if *diffs.get(&diff).unwrap_or(&0) == group[i] {
                    return format!("{:?}{:?}{:?}", 2 * group[i] - group[j], group[i], group[j]).parse::<i64>().unwrap()
                }
                diffs.insert(diff, group[j]);
            }
        }
    }
    -1
}

fn insert_digit(digit: i32, digits: i32) -> i32 {
    if digits == 0 {
        return digit
    }
    let num_digits = digits.to_string().len() as i32;
    let mut left = num_digits;
    let mut right: i32 = 0;
    let mut mid: i32;
    loop {
        mid = (left + right) / 2;
        if left == right {
            break;
        }
        let k = 10_i32.pow(mid as u32);
        let mid_digit = digits % (k * 10) / k;
        if digit < mid_digit {
            right = mid + 1;
        } else if digit > mid_digit {
            left = mid;
        } else {
            break;
        }
    }
    if mid < num_digits {
        let m = 10_i32.pow(mid as u32);
        return (digits / m) * m * 10 + digit * m + digits % m
    } else {
        return digits + digit * 10_i32.pow(num_digits as u32)
    }
}

/// sort digits in ascending order
fn simplify(mut n: i32) -> i32 {
    let mut digits = 0;
    while n > 0 {
        let digit = n % 10;
        digits = insert_digit(digit, digits);
        n /= 10;
    }
    digits
}

#[test]
pub fn test() {
    assert_eq!(simplify(4231), 1234);
    assert_eq!(simplify(2314), 1234);
    assert_eq!(simplify(2314), 1234);
}
