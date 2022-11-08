/// The following iterative sequence is defined for the set of positive
/// integers:
///   n → n/2 (n is even)
///   n → 3n + 1 (n is odd)
/// Using the rule above and starting with 13, we generate the following
/// sequence:
///   13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
/// It can be seen that this sequence (starting at 13 and finishing at 1)
/// contains 10 terms. Although it has not been proved yet (Collatz Problem),
/// it is thought that all starting numbers finish at 1.
/// Which starting number, under one million, produces the longest chain?
/// NOTE: Once the chain starts the terms are allowed to go above one million.
use std::collections::HashMap;

pub fn main() -> i64 {
    let mut map: HashMap<i64, i32> = HashMap::new();
    map.insert(1, 1);

    let mut max_chain: i32 = 0;
    let mut max_chain_start_val: i64 = 1;

    for m in 2..1_000_000 {
        let mut stack: Vec<i64> = Vec::new();
        let mut n = m;
        while !map.contains_key(&n) {
            stack.push(n);
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = 3 * n + 1;
            }
        }
        let mut i = *map.get(&n).unwrap() + 1;
        while !stack.is_empty() {
            let val = stack.pop().unwrap();
            if i > max_chain {
                max_chain = i;
                max_chain_start_val = val;
            }
            map.insert(val, i);
            i += 1;
        }
    }
    return max_chain_start_val;
}
