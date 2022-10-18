#[allow(dead_code)]
pub fn main() -> i32 {
    let mut res = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            res += i;
        }
    }
    return res;
}
