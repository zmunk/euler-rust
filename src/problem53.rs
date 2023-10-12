pub fn main() -> i32 {
    let mut acc = 0;
    for n in 23..=100 {
        let mut r = n / 2;
        let mut count = 0;
        loop {
            let mut c = 1.;
            for i in 1..=r {
                let inc = i as f64;
                c *= (n as f64 + 1. - inc) / inc;
            }
            if c > 1_000_000. {
                count += 1;
            } else {
                break;
            }
            r -= 1;
        }
        acc += 2 * count;
        if n % 2 == 0 {
            acc -= 1;
        }
    }
    acc
}
