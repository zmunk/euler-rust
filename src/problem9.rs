/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for
/// which, a^2 + b^2 = c^2
/// For example, 32 + 42 = 9 + 16 = 25 = 52.
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.

pub fn main() -> i32 {
    for a in 1..=332 {
        for b in a+1..=(1000 - a - 1) / 2 {
            let c: i32 = 1000 - a - b;
            if a * a + b * b == c * c { return a * b * c }
        }
    }
    return -1;
}
