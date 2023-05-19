/// Take the number 192 and multiply it by each of 1, 2, and 3:
/// 192 × 1 = 192
/// 192 × 2 = 384
/// 192 × 3 = 576
/// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)
/// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
/// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?
use std::collections::HashSet;

pub fn main() -> i32 {
    let mut max = 0;
    for upper_limit in 2..=9 {
        let mut n = 1;
        loop {
            let mut s = String::new();
            for i in 1..=upper_limit {
                s.push_str(&(i * n).to_string());
            }
            if s.len() < 9 {
                n += 1;
                continue;
            }
            if s.len() > 9 {
                break;
            }
            let p = s.parse::<i32>().unwrap();
            if is_pandigital(p) {
                if p > max {
                    max = p;
                }
            }
            n += 1;
        }
    }
    max
}

fn is_pandigital(num: i32) -> bool {
    if num.to_string().len() != 9 {
        return false;
    }
    let mut digits: HashSet<_> = HashSet::from_iter(
        num.to_string()
            .chars()
            .map(|a| a.to_digit(10).unwrap())
            .collect::<Vec<_>>(),
    );
    for d in 1..=9 {
        if !digits.contains(&d) {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    assert_eq!(is_pandigital(192384576), true);
    assert_eq!(is_pandigital(1192384576), false);
    assert_eq!(is_pandigital(193384576), false);
}
