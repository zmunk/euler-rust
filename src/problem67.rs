/// By starting at the top of the triangle below and moving to adjacent numbers
/// on the row below, the maximum total from top to bottom is 23.
///   3
///   7 4
///   2 4 6
///   8 5 9 3
/// That is, 3 + 7 + 4 + 9 = 23.
/// Find the maximum total from top to bottom in a 15K text
/// file containing a triangle with one-hundred rows.
use std::cmp::max;
use std::fs;

pub fn main() -> i32 {
    let contents = fs::read_to_string("data/problem67_triangle.txt").expect("unable to read file");
    let rows: std::str::Split<&str> = contents.trim().split("\n");
    let mut arr: Vec<Vec<i32>> = rows
        .map(|v| v.split(" ").map(|v| v.parse::<i32>().unwrap()).collect())
        .collect();
    let mut last_row: Vec<i32> = arr.pop().unwrap();
    let mut second_last_row: Vec<i32>;
    while last_row.len() > 1 {
        second_last_row = arr.pop().unwrap();
        for (i, v) in &mut second_last_row.iter_mut().enumerate() {
            *v += max(last_row[i], last_row[i + 1]);
        }
        last_row = second_last_row;
    }
    last_row[0]
}
