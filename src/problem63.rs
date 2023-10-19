pub fn main() -> i32 {
    let mut count = 1; // case: 1 ^ 1
    for base in 2..10 {
        let mut n: f64 = 1.;
        let mut num_digits: i32 = 1;
        for i in 1.. {
            n *= base as f64;
            while n >= 10.0 {
                n /= 10.0;
                num_digits += 1;
            }
            if num_digits.abs_diff(i) > 2 {
                break;
            } else if num_digits == i {
                count += 1;
            }
        }
    }
    count
}
