/// The decimal number, 585 = 1001001001_2 (binary), is palindromic in both bases.
/// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
/// (Please note that the palindromic number, in either base, may not include leading zeros.)
use crate::utils::*;

pub fn main() -> i32 {
    let mut res = 0;
    for i in 1..1_000_000 {
        if is_num_palindrome(i as i32) && is_str_palindrome(&dec_to_bin(i).to_string()) {
            res += i as i32;
        }
    }
    res
}

/// decimal to binary
fn dec_to_bin(mut num: u32) -> String {
    let mut res = String::new();
    while num > 0 {
        res.push_str(&(num % 2).to_string());
        num /= 2;
    }
    res.chars().rev().collect()
}

#[test]
fn test() {
    assert_eq!(dec_to_bin(4), "100");
    assert_eq!(dec_to_bin(585), "1001001001");
    assert_eq!(is_str_palindrome("mom"), true);
    assert_eq!(is_str_palindrome("mommy"), false);
}
