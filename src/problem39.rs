/// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.
/// {20,48,52}, {24,45,51}, {30,40,50}
/// For which value of p ≤ 1000, is the number of solutions maximised?
use std::f64::consts::SQRT_2;

pub fn main() -> i32 {
    let mut max = 0;
    let mut max_val = 0;
    for i in 1..=1000 {
        let p = i as f64;
        let lower_limit = ((SQRT_2 - 1.0) * p).ceil() as u32; // inclusive
        let upper_limit = (p / 2.0).ceil() as u32; // exclusive
        let mut count = 0;
        for c in lower_limit..upper_limit {
            let lower_limit = ((p - c as f64) / 2.0).ceil() as u32;
            let upper_limit = c;
            for b in lower_limit..upper_limit {
                let a = (p - c as f64 - b as f64) as u32;
                if a * a + b * b == c * c {
                    count += 1;
                }
            }
        }
        if count > max {
            max = count;
            max_val = i;
        }
    }
    max_val
}
