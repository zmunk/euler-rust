/// n! means n × (n − 1) × ... × 3 × 2 × 1
/// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the
/// digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
/// Find the sum of the digits in the number 100!
pub fn main() -> i32 {
    let mut digits: Vec<i32> = vec![1];
    for n in 2..=100 {
        let mut carry: Vec<i32> = vec![0];
        for (i, digit) in digits.iter_mut().enumerate() {
            while carry.len() < i + 1 {
                carry.push(0);
            }
            let d = *digit * n + carry[i];
            *digit = d % 10;
            let mut c = d / 10;
            let mut j = i + 1;
            while c > 0 {
                let cc = c % 10;
                if carry.len() == j {
                    carry.push(cc);
                } else {
                    carry[j] += cc;
                }
                c /= 10;
                j += 1;
            }
        }
        for i in digits.len()..carry.len() {
            digits.push(carry[i]);
            carry[i] = 0;
        }
        let mut tail = true;
        digits.retain(|i| {
            if !tail {
                return true;
            }
            if *i > 0 {
                tail = false;
                return true;
            }
            false
        });
    }
    return digits.iter().fold(0, |a, b| a + b);
}

/// for debugging
/// prints list of integers in reverse as a string
#[allow(dead_code)]
fn print_vec(name: &str, list: &Vec<i32>) {
    println!(
        "{}: {}",
        name,
        list.clone()
            .into_iter()
            .rev()
            .map(|i| i.to_string())
            .collect::<String>(),
    );
}
