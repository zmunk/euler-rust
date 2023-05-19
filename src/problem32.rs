/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
/// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
/// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
/// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
use std::collections::HashSet;

#[allow(dead_code)]
fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn main() -> i32 {
    let universal_set = HashSet::from_iter((1..=9).collect::<Vec<_>>());
    let mut res_set = HashSet::new();
    let mut res = 0;

    for outer_subset_len in [1, 2] {
        for outer_subset in get_subsets((1..=9).collect::<Vec<_>>(), outer_subset_len) {
            let remaining_set: HashSet<_> =
                universal_set.difference(&outer_subset).cloned().collect();
            for left in get_perms(Vec::from_iter(outer_subset)) {
                for subset in
                    get_subsets(Vec::from_iter(remaining_set.clone()), 5 - outer_subset_len)
                {
                    let rem = remaining_set
                        .difference(&subset)
                        .cloned()
                        .collect::<HashSet<_>>();
                    for perm in get_perms(Vec::from_iter(subset.clone())) {
                        let prod = left * perm;
                        if prod.to_string().len() != rem.len() {
                            continue;
                        }
                        if rem == num_to_set(prod) {
                            if !res_set.contains(&prod) {
                                res += prod;
                                res_set.insert(prod);
                            }
                        }
                    }
                }
            }
        }
    }

    res
}

fn num_to_set(n: i32) -> HashSet<i32> {
    HashSet::from_iter(
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32),
    )
}

fn get_subsets(arr: Vec<i32>, subset_size: i32) -> Vec<HashSet<i32>> {
    if subset_size == 0 {
        return vec![HashSet::new()];
    }
    let mut res = vec![];
    let first_value = arr.first().unwrap();
    for mut subset in get_subsets((&arr[1..]).to_vec(), subset_size - 1) {
        subset.insert(*first_value);
        res.push(subset);
    }
    if subset_size as usize <= arr.len() - 1 {
        for subset in get_subsets((&arr[1..]).to_vec(), subset_size) {
            res.push(subset);
        }
    }
    res
}

fn get_perms(arr: Vec<i32>) -> Vec<i32> {
    let binding = get_perms_as_arrays(arr);
    let res = binding
        .iter()
        .map(|v| {
            v.iter()
                .map(|a| a.to_string())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();
    res
}

fn get_perms_as_arrays(arr: Vec<i32>) -> Vec<Vec<i32>> {
    if arr.len() == 1 {
        return vec![arr.to_vec()];
    }
    let first_element = arr.first().unwrap();
    let mut res = vec![];
    for sub_arr in get_perms_as_arrays(arr[1..].to_vec()) {
        for i in 0..=sub_arr.len() {
            let mut a = sub_arr.clone();
            a.insert(i, *first_element);
            res.push(a);
        }
    }
    res
}

#[test]
fn test() {}
