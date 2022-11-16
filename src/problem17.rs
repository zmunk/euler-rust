/// If the numbers 1 to 5 are written out in words: one, two, three, four,
/// five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
/// in words, how many letters would be used?
/// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
/// forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
/// 20 letters. The use of "and" when writing out numbers is in compliance with
/// British usage.
use std::collections::HashMap;

pub fn main() -> i32 {
    let mut res: i32 = 0;
    for i in 1..=1000 {
        res += count_letters(&wordify(i));
    }
    res
}

fn count_letters(s: &str) -> i32 {
    let mut res: i32 = 0;
    for c in s.chars() {
        if c != ' ' {
            res += 1;
        }
    }
    res
}

fn wordify(n: i32) -> String {
    let map: HashMap<i32, &str> = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ]);
    let mut n = n;
    let mut res = String::new();
    if n >= 1000 {
        res.push_str(*map.get(&(n / 1000)).unwrap());
        res.push_str(" thousand");
        n %= 1000;
        if n > 0 {
            res.push_str(" ");
        }
    }
    if n >= 100 {
        res.push_str(*map.get(&(n / 100)).unwrap());
        res.push_str(" hundred");
        n %= 100;
        if n > 0 {
            res.push_str(" and ");
        }
    }
    if n >= 20 {
        let ones_digit = n % 10;
        res.push_str(*map.get(&(n - ones_digit)).unwrap());
        n = ones_digit;
        if n > 0 {
            res.push_str(" ");
        }
    }
    if n > 0 {
        res.push_str(*map.get(&n).unwrap());
    }
    res
}
