#[allow(dead_code)]
pub fn main() -> i64 {
    let mut n: i64 = 600851475143;
    let primes = get_primes_less_than((n as f64).sqrt().ceil() as usize);

    let mut largest_prime = 0;
    loop {
        let prime = get_smallest_prime_factor(n, &primes);
        if prime > largest_prime { largest_prime = prime }
        if prime == n { break }
        n = n / prime;
    }

    return largest_prime;
}

/// Returns vector of primes less than n
/// Uses sieve of eratosthenes
#[allow(dead_code)]
fn get_primes_less_than(n: usize) -> Vec<i64> {
    let n = n + 1;
    let mut flags: Vec<bool> = vec![true; n]; // true -> prime
    flags[0] = false;
    flags[1] = false;

    let mut prime = 2;
    while prime*prime < n {
        let mut i = prime*prime;
        while i < n {
            flags[i] = false;
            i += prime;
        }
        prime += 1;
        while prime < n && !flags[prime] {
            prime += 1;
        }
    }

    let mut primes: Vec<i64> = vec![];
    for (i, flag) in flags.iter().enumerate() {
        if *flag { primes.push(i as i64) }
    }

    return primes;
}

/// Iterates through all primes from 2 to the square root of n
/// until it finds a prime that factors into n
/// then it returns this prime.
/// Returns n if no prime found that factors into n.
///
/// # Arguments
/// * n - the value whose prime factor we are looking for
/// * primes - a vector containing prime numbers
#[allow(dead_code)]
fn get_smallest_prime_factor(n: i64, primes: &Vec<i64>) -> i64 {
    for p in primes {
        if p*p > n { break }
        if n % p == 0 { return *p }
    }
    n // if no primes divide it, n is prime
}
