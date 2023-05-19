/// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
/// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
/// Note: As 1! = 1 and 2! = 2 are not sums they are not included.

pub fn main() -> i32 {
    let mut res = 0;
    for i in 3..100_000 {
        let sum = sum_fact_dig(i);
        if i as u128 == sum {
            res += sum;
        }
    }
    res as i32
}

/// Returns sum of factorial of digits of num
fn sum_fact_dig(num: u32) -> u128 {
    let mut n = num;
    let mut res = 0;
    while n > 0 {
        res += factorial((n % 10).into());
        n /= 10;
    }
    res
}

fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

#[test]
fn test() {}
