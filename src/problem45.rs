use num::integer::Roots;

pub fn main() -> i64 {
    for n in 286.. {
        let i = n * (n + 1) / 2;
        if is_pentagonal(i) && is_hexagonal(i) {
            return i
        }
    }
    -1
}

#[allow(dead_code)]
fn is_triangle(num: i64) -> bool {
    let a = 2 * num;
    let b = a.sqrt();
    a == b * (b + 1)
}

fn is_hexagonal(num: i64) -> bool {
    let a = 2 * num;
    let b = a.sqrt();
    b % 2 == 1 && a == b * (b + 1)
}

fn is_pentagonal(num: i64) -> bool {
    let a = 6 * num;
    let b = a.sqrt();
    b % 3 != 0 && a == b * (b + 1)
}
