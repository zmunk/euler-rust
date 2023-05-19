use crate::utils::*;

#[allow(dead_code)]
pub fn main() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    'outer: for total in 2.. {
        a = total / 2;
        b = total - a;
        loop {
            if is_num_palindrome((1000 - a) * (1000 - b)) {
                break 'outer;
            }
            if a == 1 {
                break;
            }
            b += 1;
            a -= 1;
        }
    }
    return (1000 - a) * (1000 - b);
}

#[test]
pub fn test() {
    assert_eq!(main(), 906609);
}
