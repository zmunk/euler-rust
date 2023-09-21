pub fn main() -> u64 {
    // let mut digits: Vec<u32> = (0..10).collect();
    let mut digits: Vec<u32> = (0..4).collect();
    let mut permutations = Vec::new();
    let n = digits.len();


    // generate_permutations(&mut digits, digits.len(), &mut permutations);
    generate_permutations(&mut digits, n, &mut permutations);
    return 1;

    let mut total: u64 = 0;

    println!("Total permutations: {}", permutations.len());
    for p in permutations {
        if p[5] != 0 && p[5] != 5 {
            continue;
        }
        if p[3] % 2 != 0 {
            continue;
        }
        if create_number(p[2], p[3], p[4]) % 3 != 0 {
            continue;
        }
        if create_number(p[4], p[5], p[6]) % 7 != 0 {
            continue;
        }
        if create_number(p[5], p[6], p[7]) % 11 != 0 {
            continue;
        }
        if create_number(p[6], p[7], p[8]) % 13 != 0 {
            continue;
        }
        if create_number(p[7], p[8], p[9]) % 17 != 0 {
            continue;
        }
        let mut number: u64 = 0;

        for digit in &p {
            number = number * 10 + u64::from(*digit);
        }
        println!("p: {:?}", p);
        println!("number: {:?}", number);
        total += number;
    }
    total.into()
}

fn create_number(a: u32, b: u32, c: u32) -> u32 {
    100 * a + 10 * b + c
}

// fn get_permutations(digits: Vec<u8>) -> Vec<Vec<u8>> {
    // if digits.len() == 1 {
        // vec![digits]
    // }

// }

fn generate_permutations(digits: &mut Vec<u32>, n: usize, result: &mut Vec<Vec<u32>>) {
    let prefix = "";//.repeat(3 - n);
    // println!("{}digits: {:?}, n: {:?}", prefix, digits, n);
    if n == 1 {
        println!("digits: {:?}", digits);
        result.push(digits.clone());
        return;
    }

    for i in 0..n {
        // println!("{}i: {:?}, n: {:?}", prefix, i, n);
        generate_permutations(digits, n - 1, result);
        // println!("{}i: {:?}, n: {:?}", prefix, i, n);

        // println!("{}digits: {:?}", prefix, digits);
        if n % 2 == 0 {
            // println!("{}swapping {:?} <-> {:?} (a)", prefix, i, n - 1);
            digits.swap(i, n - 1);
        } else {
            // println!("{}swapping {:?} <-> {:?} (b)", prefix, 0, n - 1);
            digits.swap(0, n - 1);
        }
        // println!("{}digits: {:?}", prefix, digits);
        // println!();
    }
}

