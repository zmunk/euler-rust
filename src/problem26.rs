/// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
/// 1/2	= 0.5
/// 1/3 = 0.(3)
/// 1/4 = 0.25
/// 1/5 = 0.2
/// 1/6 = 0.1(6)
/// 1/7 = 0.(142857)
/// 1/8 = 0.125
/// 1/9 = 0.(1)
/// 1/10 = 0.1
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
/// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
use std::collections::HashMap;

pub fn main() -> i32 {
    let mut mx = 0;
    let mut mxd = -1;
    for denom in 2..=1000 {
        let rep = get_num_repeating_digits(denom);
        if rep > mx {
            mx = rep;
            mxd = denom;
        }
    }
    mxd
}

#[test]
fn test() {
    assert_eq!(get_num_repeating_digits(2), 0);
    assert_eq!(get_num_repeating_digits(7), 6);
}

fn get_num_repeating_digits(n: i32) -> i32 {
    let mut tuples: HashMap<(i32, i32), i32> = HashMap::new();
    let mut r = 1;
    let mut shifted;
    let mut q;
    let denom = n;

    let mut i = 0;
    loop {
        shifted = r * 10;
        q = shifted / denom;
        r = shifted % denom;
        if tuples.contains_key(&(q, r)) {
            let ii = tuples.get(&(q, r)).unwrap();
            let mut rep = i - ii;
            if (q, r) == (0, 0) {
                rep = 0;
            }
            return rep;
        }
        tuples.insert((q, r), i);
        i += 1;
    }
}
