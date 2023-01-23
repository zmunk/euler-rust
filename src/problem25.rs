/// The Fibonacci sequence is defined by the recurrence relation:
///    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
/// Hence the first 12 terms will be:
///    F1 = 1
///    F2 = 1
///    F3 = 2
///    F4 = 3
///    F5 = 5
///    F6 = 8
///    F7 = 13
///    F8 = 21
///    F9 = 34
///    F10 = 55
///    F11 = 89
///    F12 = 144
/// The 12th term, F12, is the first term to contain three digits.
/// What is the index of the first term in the Fibonacci sequence to contain
/// 1000 digits?
pub fn main() -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut i: i32 = 1;
    let mut n_digits: i32 = 6;
    let thresh = 10_i32.pow(n_digits as u32);
    while n_digits < 1000 {
        let t = a;
        a = b;
        b += t;
        i += 1;
        if a > thresh && b > thresh {
            a /= 10;
            b /= 10;
            n_digits += 1;
        }
    }
    return i;
}
