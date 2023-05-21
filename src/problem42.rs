/// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
/// By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.
/// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?
use std::collections::HashMap;
use std::fs;

pub fn main() -> i32 {
    let contents = fs::read_to_string("data/problem42_words.txt").expect("unable to read file");
    let binding = contents.trim().replace("\"", "");
    let words: Vec<&str> = binding.split(",").collect();
    let char_to_num = get_char_map();

    let mut count = 0;
    for word in words {
        let mut acc: u32 = 0;
        for c in word.chars() {
            acc += char_to_num[&c] as u32;
        }
        if is_triangle(acc) {
            count += 1;
        }
    }
    count
}

fn get_char_map() -> HashMap<char, u8> {
    HashMap::from_iter(
        "_ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .map(|(a, b)| (b, a as u8))
            .collect::<Vec<(char, u8)>>(),
    )
}

fn is_triangle(i: u32) -> bool {
    let a = (i * 2) as f32;
    (a + a.sqrt().ceil()).sqrt() % 1.0 == 0.0
}

#[test]
fn test_triangle() {
    assert_eq!(is_triangle(1), true);
    assert_eq!(is_triangle(3), true);
    assert_eq!(is_triangle(6), true);
    assert_eq!(is_triangle(10), true);
    assert_eq!(is_triangle(55), true);
    assert_eq!(is_triangle(5), false);
    assert_eq!(is_triangle(12), false);
}

#[test]
fn test_char() {
    let char_to_num = get_char_map();
    assert_eq!(char_to_num[&'S'], 19);
}
