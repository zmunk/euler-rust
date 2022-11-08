/// Starting in the top left corner of a 2×2 grid, and only being able to move
/// to the right and down, there are exactly 6 routes to the bottom right
/// corner.
/// How many such routes are there through a 20×20 grid?
pub fn main() -> i64 {
    let n: i32 = 40;
    let mut res: u128 = 1;
    for i in 21..=n {
        res *= i as u128;
    }
    for i in 1..=n / 2 {
        res /= i as u128;
    }
    return res as i64;
}
