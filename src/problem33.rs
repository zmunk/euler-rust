/// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
/// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
/// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
/// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

pub fn main() -> i32 {
    let mut num = 1;
    let mut denom = 1;
    for mm in 10..100 {
        if mm % 10 == 0 {
            continue;
        }
        for nn in mm + 1..100 {
            for digit in [mm / 10, mm % 10] {
                let m = remove_digit(mm, digit);
                let n = remove_digit(nn, digit);
                if n == -1 {
                    continue;
                }
                if (m as f64) / (n as f64) == (mm as f64) / (nn as f64) {
                    println!("{:?} / {:?} = {:?} / {:?}", mm, nn, m, n);
                    num *= m;
                    denom *= n;
                }
            }
        }
    }
    denom / num
}

fn remove_digit(n: i32, digit: i32) -> i32 {
    if n % 10 == digit {
        return n / 10;
    }
    if n / 10 == digit {
        return n % 10;
    }
    -1
}

#[test]
fn test() {
    assert_eq!(remove_digit(98, 7), -1);
    assert_eq!(remove_digit(98, 9), 8);
}
