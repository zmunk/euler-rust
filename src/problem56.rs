pub fn main() -> i32 {
    let mut max: i32 = 0;
    for b in 2..100 {
        let mut num = vec![1];
        let base = to_vec(b);
        for _ in 0..99 {
            num = product(num, base.clone());
            let sum = num.iter().fold(0, |acc, x| acc + x);
            if sum as i32 > max {
                max = sum as i32;
            }
        }
    }
    max
}

fn to_vec(n: i32) -> Vec<u16> {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap() as u16).collect()
}

fn product(mut a: Vec<u16>, mut b: Vec<u16>) -> Vec<u16> {
    a.reverse();
    b.reverse();
    let mut res = vec![];
    let mut carry = 0;
    for k in 0.. {
        let mut prod = carry;
        let mut never_entered = true;
        for i in 0..=k {
            if i >= a.len() || k - i >= b.len() {
                continue;
            }
            never_entered = false;
            prod += a[i] * b[k - i];
        }
        if prod == 0 && never_entered {
            break;
        }
        res.push(prod % 10);
        carry = prod / 10;
    }
    res.reverse();
    res
}
