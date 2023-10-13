use std::collections::HashSet;

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

    let mut compatible_pairs = vec![];
    let mut compatible_triplets  = vec![];
    let mut compatible_quads  = vec![];

    let mut i = 0;
    loop {
        let p1 = primes[i];
        if p1 * p1 > primes_lim {
            panic!("not enough primes")
        }
        for &p2 in &primes[..i] {
            if check_compatible(p1, p2) {
                compatible_pairs.push((p1, p2));
            }
        }

        for &(a, b) in compatible_pairs.iter() {
            if check_compatible(a, p1) && check_compatible(b, p1) {
                compatible_triplets.push((a, b, p1));
            }
        }

        for triplet in &compatible_triplets {
            let mut compatible = true;
            let (a, b, c) = triplet.clone();
            for p in [a, b, c] {
                if !check_compatible(p, p1) {
                    compatible = false;
                    break;
                }
            }
            if compatible {
                compatible_quads.push((a, b, c, p1));
            }
        }

        for quad in &compatible_quads {
            let mut compatible = true;
            let (a, b, c, d) = quad.clone();
            for p in [a, b, c, d] {
                if !check_compatible(p, p1) {
                    compatible = false;
                    break;
                }
            }
            if compatible {
                return [a, b, c, d, p1].iter().sum::<i64>() as i32
            }
        }
        i += 1;
    }
}

