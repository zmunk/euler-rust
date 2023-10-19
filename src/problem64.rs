use std::collections::HashSet;

pub fn main() -> i32 {
    let mut count = 0;
    for n in 2..10_000 {
        if get_period(n) % 2 == 1 {
            count += 1;
        }
    }
    count
}

pub fn get_period(n: i32) -> i32 {
    let sqrt = (n as f64).sqrt();

    let mut a = -1;
    let mut c = 1;
    let mut b = sqrt.floor() as i32;

    let mut set = HashSet::new();
    loop {
        if set.contains(&(a, b, c)) {
            break
        }
        set.insert((a, b, c));
        c = (n - b.pow(2)) / c;
        if c == 0 {
            return 0
        }
        a = ((sqrt + b as f64) / c as f64).floor() as i32;
        b = a * c - b;
    }
    set.len() as i32 - 1
}


