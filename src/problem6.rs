/// The sum of the squares of the first ten natural numbers is,
/// 1^2 + 2^2 + ... + 10^2 = 385
/// The square of the sum of the first ten natural numbers is,
/// (1 + 2 + ... + 10)^2 = 55^2 = 3025
/// Hence the difference between the sum of the squares of the first ten
/// natural numbers and the square of the sum is
/// 3025 - 385 = 2640
/// Find the difference between the sum of the squares of the first one hundred
/// natural numbers and the square of the sum.
pub fn main() -> i32 {
    let n = 100;
    let mut sum_sq: i32 = 0; // sum of squares
    let mut sum: i32 = 0;
    for i in 1..=n {
        sum += i;
        sum_sq += i * i;
    }
    let sq_sum = sum * sum; // square of sum
    return sq_sum - sum_sq;
}
