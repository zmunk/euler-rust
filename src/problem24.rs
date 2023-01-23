/// A permutation is an ordered arrangement of objects. For example, 3124 is
/// one possible permutation of the digits 1, 2, 3 and 4. If all of the
/// permutations are listed numerically or alphabetically, we call it
/// lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
///    012   021   102   120   201   210
/// What is the millionth lexicographic permutation of the digits
/// 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
pub fn main() -> String {
    let mut v: i32 = 1e6 as i32;
    let mut digits: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut res: Vec<i8> = Vec::new();
    for i in (1..=9).rev() {
        let f = factorial(i);
        res.push(digits.remove(((v - 1) / f) as usize));
        v = (v - 1) % f + 1;
    }
    res.push(digits.pop().unwrap());
    return res.iter().map(|i| i.to_string()).collect::<String>();
}

fn factorial(v: i32) -> i32 {
    let mut res: i32 = 1;
    for i in 2..=v {
        res *= i;
    }
    res
}
