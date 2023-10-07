use crate::utils::get_primes_less_than;

pub fn main() -> i64 {
    let prime_lim = 1000_000;
    let primes = get_primes_less_than(prime_lim);

    let k = 4;
    let mut in_a_row = 0;
    for i in 1.. {
        let mut count = 0;
        let mut n = i;

        for p in &primes {
            let mut divisible = false;
            while n % p == 0 {
                divisible = true;
                n /= p;
            }
            if divisible {
                count += 1;
            }
            if n == 1 {
                break;
            }
        }
        if n != 1 {
            panic!("not enough primes");
        }
        if count == k {
            in_a_row += 1;
        } else {
            in_a_row = 0;
        }
        if in_a_row == k {
            return i - k + 1;
        }
    }
    -1
}

