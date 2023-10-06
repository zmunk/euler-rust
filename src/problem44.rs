use std::collections::HashMap;
use num::integer::Roots;

pub fn main() -> i64 {
    let mut map: HashMap<i32, (i64, i64)> = HashMap::new();
    let mut max_key: i32 = 0;
    map.insert(max_key, (0, 1));
    loop {
        let coords: (i64, i64) = (0, (max_key + 2).into());
        let diff = get_diff(&coords);
        if check(diff, &coords) {
            return diff
        }
        let lim = diff;
        for i in 0..=max_key {
            let mut coords = *map.get(&i).unwrap();
            loop {
                coords = (coords.0 + 1, coords.1 + 1);
                let diff = get_diff(&coords);
                if check(diff, &coords) {
                    return diff
                }
                if diff >= lim  {
                    break;
                }
            }
            map.insert(i, coords);
        }
        max_key += 1;
        map.insert(max_key, coords);
    }
}

fn check(diff: i64, coords: &(i64, i64)) -> bool {
    is_pentagonal(diff) && is_pentagonal(get_sum(coords))
}

fn get_sum(coords: &(i64, i64)) -> i64 {
    let a = coords.0 + 1;
    let b = coords.1 + 1;
    ((b * (3 * b - 1) + a * (3 * a - 1)) / 2).into()
}

fn get_diff(coords: &(i64, i64)) -> i64 {
    let a = coords.0 + 1;
    let b = coords.1 + 1;
    ((b * (3 * b - 1) - a * (3 * a - 1)) / 2).into()
}

fn is_pentagonal(num: i64) -> bool {
    let a = 6 * num;
    let b = a.sqrt();
    b % 3 != 0 && a == b * (b + 1)
}
