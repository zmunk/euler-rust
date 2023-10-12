use std::collections::HashMap;

use crate::utils::get_primes_less_than;

pub fn main() -> i64 {
    let mut collection = HashMap::new();

    let num_digits = 6;
    let combos = (1..2_i32.pow(num_digits)).map(|i| {
        let s = format!("{:01$b}", i, num_digits as usize);
        s.match_indices("1").map(|(i, _)| i).collect::<Vec<_>>()
    });

    let primes = get_primes_less_than(1_000_000);

    let low: i64 = 10_i64.pow(num_digits - 1);
    let up: i64 = 10 * low;
    for number in primes {
        if number < low || number >= up {
            continue;
        }
        let number_string = number.to_string();
        let number_vec: Vec<u8> = number_string.chars().map(|d| d.to_digit(10).unwrap() as u8).collect();

        let index_list = combos.clone().filter(|indices| {
            if indices.len() == 1 {
                return true;
            }
            let digit = number_vec[indices[0]];
            for &other_index in indices[1..].iter() {
                if digit != number_vec[other_index] {
                    return false;
                }
            }
            true
        });

        for indices in index_list {
            let mut s = number_string.clone();
            for i in indices {
                s.replace_range(i..i+1, "_");
            }
            let entry = collection.entry(s).or_insert(vec![]);
            entry.push(number.clone());
            if entry.len() == 8 {
                return *entry.iter().min().unwrap()
            }
        }
    }
    -1
}

