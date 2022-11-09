/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
/// What is the sum of the digits of the number 2^1000?
pub fn main() -> i32 {
    let n: i32 = 1000;
    let mut arr: Vec<i32> = vec![1];

    for _ in 0..n {
        let mut carry: i32 = 0;
        for v in &mut arr {
            let mut val = 2 * *v + carry;
            carry = val / 10;
            *v = val % 10;
        }
        if carry > 0 {
            arr.push(carry);
        }
    }
    let sum: i32 = arr.iter().fold(0, |a, b| a + b);
    return sum;
}

#[allow(dead_code)]
fn to_int(arr: &Vec<i32>) -> i32 {
    arr.iter()
        .rev()
        .map(|&a| a.to_string())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}
