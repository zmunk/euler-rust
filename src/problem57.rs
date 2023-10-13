pub fn main() -> i32 {
    let mut a = 3;
    let mut b = 2;
    let mut count = 0;
    for _ in 0..1000 {
        (a, b) = (a + 2 * b, a + b);
        if a.to_string().len() > 8 {
            let d = 10_i32.pow((a.to_string().len() - 7) as u32);
            (a, b) = (a / d, b / d);
        }
        if a.to_string().len() > b.to_string().len() {
            count += 1;
        }
    }
    count
}
