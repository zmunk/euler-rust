#[allow(dead_code)]
pub fn main() -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut sum = 0;
    while b < 4_000_000 {
        let c = a + b;
        a = b;
        b = c;
        if a % 2 == 0 {
            sum += a;
        }
    }
    return sum;
}
