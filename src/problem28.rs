/// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
/// It can be verified that the sum of the numbers on the diagonals is 101.
/// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

pub fn main() -> i32 {
    let side_len = 1001;
    sum_diag(side_len)
}

fn sum_diag(side_len: i32) -> i32 {
    let n = (side_len - 1) / 2;
    let res = 1 + 4 * n * (2 * n + 1) * (2 * n - 1) / 3 + 10 * n * (n + 1);
    res
}

#[test]
fn test() {
    assert_eq!(sum_diag(5), 101);
}
