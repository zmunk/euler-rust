/// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
/// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
/// It is possible to make £2 in the following way:
/// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
/// How many different ways can £2 be made using any number of coins?

pub fn main() -> i32 {
    get_num_combos(200, &[200, 100, 50, 20, 10, 5, 2])
}

fn get_num_combos(n: i32, arr: &[i32]) -> i32 {
    if arr.len() == 0 || n == 0 {
        return 1;
    }
    let biggest_denom = arr.first().unwrap();
    let shrunken_arr = &arr[1..];
    let mut n_combos = 0;
    let mut rem = n;
    while rem >= 0 {
        n_combos += get_num_combos(rem, &shrunken_arr);
        rem -= biggest_denom;
    }
    n_combos
}


#[test]
fn test() {
    assert_eq!(get_num_combos(10, &[2]), 6);
    assert_eq!(get_num_combos(5, &[5, 2]), 4);
}
