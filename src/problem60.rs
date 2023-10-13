use std::collections::{HashSet, HashMap};
use crate::utils::get_primes_less_than;

pub fn main() -> i32 {
    let primes_lim = 100_000_000;
    let primes = get_primes_less_than(primes_lim);
    let primes_set: HashSet<i64> = HashSet::from_iter(primes.iter().cloned());
    println!("primes generated");

    let check_prime = |n| {
        if n > primes_lim {
            panic!("not enough primes")
        }
        primes_set.contains(&n)
    };

    let check_compatible = |p1: i64, p2: i64| {
        let concat1 = format!("{p1}{p2}").parse::<i64>().unwrap();
        let concat2 = format!("{p2}{p1}").parse::<i64>().unwrap();
        check_prime(concat1) && check_prime(concat2)
    };

    let mut compatible: HashMap<i64, Vec<Vec<i64>>> = HashMap::new();

    for i in 0.. {
        let p1 = primes[i];
        if p1 * p1 > primes_lim {
            panic!("not enough primes")
        }

        let mut list_compat: Vec<Vec<i64>> = vec![];
        let mut compatible_values = HashSet::new();

        for &p2 in &primes[..i] {
            if !check_compatible(p1, p2) {
                continue;
            }
            compatible_values.insert(p2);
            list_compat.push(vec![p2]);
            if !compatible.contains_key(&p2) {
                continue;
            }
            for compat in compatible.get(&p2).unwrap() {
                if HashSet::from_iter(compat.iter().cloned()).is_subset(&compatible_values) {
                    if compat.len() == 3 {
                        return (compat.iter().sum::<i64>() + p1 + p2) as i32
                    }
                    list_compat.push(compat.iter().chain([&p2]).cloned().collect());
                }
            }
        }
        compatible.insert(p1, list_compat);
    }
    0
}

