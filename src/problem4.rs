#[allow(dead_code)]
pub fn main() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    'outer: for total in 2.. {
        a = total / 2;
        b = total - a;
        loop {
            if is_palindrome((1000 - a) * (1000 - b)) { break 'outer }
            if a == 1 { break }
            b += 1;
            a -= 1;
        }
    }
    return (1000 - a) * (1000 - b);
}

#[allow(dead_code)]
fn is_palindrome(n: i32) -> bool {
    let s: &str = &n.to_string();
    let r: &str = &s.chars().rev().collect::<String>();
    return s == r;
}
