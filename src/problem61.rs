use std::collections::{HashMap, HashSet};
use num::integer::Roots;

pub fn main() -> i32 {
    let mut map: HashMap<u32, Vec<(Vec<u32>, HashSet<i32>)>> = HashMap::new();
    for i in 1000..10_000 {
        if i % 100 < 10 {
            continue;
        }
        for (func, n) in [
            (is_triangle as fn(u32) -> bool, 3),
            (is_square, 4),
            (is_pentagonal, 5),
            (is_hexagonal, 6),
            (is_heptagonal, 7),
            (is_octagonal, 8),
        ] {
            if func(i) {
                map.entry(i / 100).or_insert(vec![]).push((vec![i % 100], HashSet::from([n])));
            }
        }
    }

    let single_inc_map = map.clone();
    for _ in 0..5 {
        map = extend(&map, &single_inc_map);
    }
    let mut acc: i32 = 0;
    for (key, val) in &map {
        for (v, _) in val {
            if v.last().unwrap() == key {
                acc += *key as i32;
            }
        }
    }
    acc * 101
}

fn is_triangle(i: u32) -> bool {
    let a = 2 * i;
    let b = a.sqrt();
    a == b * (b + 1)
}

fn is_square(i: u32) -> bool {
    let a = i.sqrt();
    i == a * a
}

fn is_pentagonal(num: u32) -> bool {
    let a = 6 * num;
    let b = a.sqrt();
    b % 3 != 0 && a == b * (b + 1)
}

fn is_hexagonal(num: u32) -> bool {
    let a = 2 * num;
    let b = a.sqrt();
    b % 2 == 1 && a == b * (b + 1)
}

fn is_heptagonal(num: u32) -> bool {
    let a = 40 * num + 9;
    let b = a.sqrt();
    b * b == a && (b + 3) % 10 == 0
}

fn is_octagonal(num: u32) -> bool {
    let a = 12 * num + 4;
    let b = a.sqrt();
    b * b == a && (b + 2) % 6 == 0
}

fn extend(
    map: &HashMap<u32, Vec<(Vec<u32>, HashSet<i32>)>>,
    single_inc_map: &HashMap<u32, Vec<(Vec<u32>, HashSet<i32>)>>,
    ) -> HashMap<u32, Vec<(Vec<u32>, HashSet<i32>)>> {
    let mut new_map: HashMap<u32, Vec<(Vec<u32>, HashSet<i32>)>> = HashMap::new();
    for (a, nodes) in map {
        for (nodes_a, seen_a) in nodes {
            let key = nodes_a.last().unwrap();
            if !single_inc_map.contains_key(&key) {
                continue;
            }
            for (nodes_b, seen_b) in single_inc_map.get(&key).unwrap() {
                if seen_a.intersection(seen_b).count() == 0 {
                    let chain: Vec<u32> = vec![nodes_a.clone(), nodes_b.clone()].concat();
                    let mut union = seen_a.clone();
                    union.extend(&*seen_b);
                    new_map.entry(*a).or_insert(vec![]).push((chain, union));
                }
            }
        }
    }
    new_map
}

