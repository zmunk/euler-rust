/// The series, 11 + 22 + 33 + ... + 1010 = 10405071317.
/// Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

pub fn main() -> u64 {
    let mut res = 0;
    for i in 1..=1000 {
        res += trunc_pow(i, i as u16);
    }
    res % 10_000_000_000
}

/// take base.pow(exp) and return last 10 digits
fn trunc_pow(base: u64, exp: u16) -> u64 {
    let mut res: u64 = 1;
    for _ in 0..exp {
        res = (res * base) % 10_000_000_000;
    }
    res
}
