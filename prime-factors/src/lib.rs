use std::collections::HashMap;

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut max_factor = n;
    let mut seen_primes = HashMap::new();

    while max_factor > 1 {
        let inner_factor = max_factor;
        for i in 2..=inner_factor {
            let seen_prime = seen_primes.entry(i).or_insert(is_prime(i));

            if *seen_prime && max_factor.is_multiple_of(i) {
                max_factor /= i;
                prime_factors.push(i);
                break;
            }
        }
    }
    prime_factors
}

fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    for i in 2..(n as f64).sqrt() as u64 + 1 {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}
