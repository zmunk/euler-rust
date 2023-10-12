pub fn main() -> i32{
    for n in 1.. {
        let s = simplify(n);
        if s == simplify(2 * n) &&
            s == simplify(3 * n) &&
            s == simplify(4 * n) &&
            s == simplify(5 * n) &&
            s == simplify(6 * n)
        {
            return n;
        }
    }
    -1
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
