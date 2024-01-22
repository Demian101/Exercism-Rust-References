pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieved: Vec<bool> = (0..=upper_bound).map(|_| true).collect();
    (2..=upper_bound).filter(|&p| {
        let is_prime = sieved[p as usize];
        (1..=upper_bound / p).for_each(|i| sieved[(i * p) as usize] = false);
        is_prime
    }).collect()
}

// edition = "2021"
// name = "sieve"
// version = "1.1.0"