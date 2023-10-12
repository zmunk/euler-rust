pub fn main() -> i32 {
    let mut count = 0;
    for n in 1..10_000 {
        if is_lychrel(n) {
            count += 1;
        }
    }
    count
}

fn iterate(arr: Vec<u8>) -> Vec<u8> {
    let mut new: Vec<u8> = vec![];
    let mut rem = 0;
    for i in 0..arr.len() {
        let res = rem + arr[i] + arr[arr.len() - 1 - i];
        new.insert(0, res % 10);
        rem = res / 10;
    }
    if rem > 0 {
        new.insert(0, rem);
    }
    new
}

fn is_palindrome(arr: &Vec<u8>) -> bool {
    for i in 0..arr.len() / 2 {
        if arr[i] != arr[arr.len() - 1 - i] {
            return false
        }
    }
    true
}

fn is_lychrel(n: i32) -> bool {
    let mut arr: Vec<u8> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    for _ in 0..50 {
        arr = iterate(arr);
        if is_palindrome(&arr) {
            return false;
        }
    }
    true
}

