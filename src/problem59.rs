use std::fs;

fn decrypt(enc: &Vec<u8>, key: &[u8; 3]) -> String {
    let mut dec = String::new();
    for (i, n) in enc.iter().enumerate() {
        let c = (n ^ key[i % 3]) as char;
        dec.push(c);
    }
    dec
}

pub fn main() -> i32 {
    let contents = fs::read_to_string("data/problem59.txt")
        .expect("unable to read file");
    let binding = contents.trim().replace("\"", "");
    let numbers: Vec<_> = binding.split(",").map(|a| a.parse::<u8>().unwrap()).collect();
    let range = ('a' as u8)..=('z' as u8);
    let mut dec = String::new();
    'outer: for i in range.clone() {
        for j in range.clone() {
            for k in range.clone() {
                dec = decrypt(&numbers, &[i as u8, j as u8, k as u8]);
                if dec.contains(" the ") {
                    break 'outer;
                }
            }
        }
    }
    println!("{:?}", dec);
    let acc: i32 = dec.chars().map(|c| c as i32).sum();
    acc
}
