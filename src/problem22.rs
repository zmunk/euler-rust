/// Using names.txt, a 46K text file containing over five-thousand first names,
/// begin by sorting it into alphabetical order. Then working out the
/// alphabetical value for each name, multiply this value by its alphabetical
/// position in the list to obtain a name score.
/// For example, when the list is sorted into alphabetical order, COLIN, which
/// is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So,
/// COLIN would obtain a score of 938 × 53 = 49714.
/// What is the total of all the name scores in the file?
use std::fs;

pub fn main() -> i32 {
    let contents = fs::read_to_string("data/problem22.txt").expect("unable to read file");
    let binding = contents.trim().replace("\"", "");
    let mut names: Vec<&str> = binding.split(",").collect();
    names.sort();
    let mut res = 0;
    for (i, name) in names.iter().enumerate() {
        res += ((i + 1) as i32) * get_name_score(&name);
    }
    return res;
}

fn get_name_score(name: &str) -> i32 {
    let mut res = 0;
    for c in name.chars() {
        res += (c as u32 - 64) as i32;
    }
    return res;
}
