/// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
/// 1634 = 1⁴ + 6⁴ + 3⁴ + 4⁴
/// 8208 = 8⁴ + 2⁴ + 0⁴ + 8⁴
/// 9474 = 9⁴ + 4⁴ + 7⁴ + 4⁴
/// As 1 = 14 is not a sum it is not included.
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

pub fn main() -> i32 {
    let mut res = 0;
    for n in 2..1_000_000 {
        if n == sum_dig_pow(n, 5) {
            res += n;
        }
    }
    res
}

fn sum_dig_pow(num: i32, exp: u32) -> i32 {
    let mut n = num;
    let mut res = 0;
    while n > 0 {
        res += (n % 10).pow(exp);
        n = n / 10;
    }
    res
}

#[test]
fn test() {
    assert_eq!(sum_dig_pow(1634, 4), 1634);
    assert_eq!(sum_dig_pow(8208, 4), 8208);
    assert_eq!(sum_dig_pow(9474, 4), 9474);
}
