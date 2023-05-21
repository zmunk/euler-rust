/// An irrational decimal fraction is created by concatenating the positive integers:
/// 0.123456789101112131415161718192021...
/// It can be seen that the 12th digit of the fractional part is 1.
/// If dn represents the nth digit of the fractional part, find the value of the following expression.
/// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

pub fn main() -> u32 {
    let n = 1;
    let mut res = 1;
    for i in 0..=6 {
        res *= get_nth_dig(10_u32.pow(i));
    }
    res
}

fn get_nth_dig(n: u32) -> u32 {
    let mut i = 0;
    let mut prev_lim = 0;
    let mut lim = 0;
    while lim < n {
        i += 1;
        prev_lim = lim;
        lim += 9 * i * 10_u32.pow(i - 1);
    }
    let m = (n - prev_lim + i - 1) / i + 10_u32.pow(i - 1) - 1;
    let r = (n - prev_lim - 1) % i;
    let d = (m % 10_u32.pow(i - r)) / 10_u32.pow(i - r - 1);
    d
}
