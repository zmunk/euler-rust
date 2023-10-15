use std::collections::HashMap;

pub fn main() -> i64 {
    let mut counts = HashMap::new();
    for i in 1.. {
        let cube = i * i * i;
        let simp = simplify(cube);
        let entry = counts.entry(simp).or_insert((cube, 0));
        *entry = (entry.0, entry.1 + 1);
        if entry.1 == 5 {
            return entry.0
        }
    }
    0
}

/// sort digits in ascending order
fn simplify(mut n: i64) -> String {
    let len = n.to_string().len();
    let mut digits = 0;
    while n > 0 {
        let digit = n % 10;
        digits = insert_digit(digit, digits);
        n /= 10;
    }
    format!("{:01$}", digits, len)
}

fn insert_digit(digit: i64, digits: i64) -> i64 {
    if digits == 0 {
        return digit
    }
    let num_digits = digits.to_string().len() as i64;
    let mut left = num_digits;
    let mut right: i64 = 0;
    let mut mid: i64;
    loop {
        mid = (left + right) / 2;
        if left == right {
            break;
        }
        let k = 10_i64.pow(mid as u32);
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
        let m = 10_i64.pow(mid as u32);
        return (digits / m) * m * 10 + digit * m + digits % m
    } else {
        return digits + digit * 10_i64.pow(num_digits as u32)
    }
}
