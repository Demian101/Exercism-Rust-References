
pub fn factors(mut n: u64) -> Vec<u64> {
    use std::time::Instant;
    let now = Instant::now();

    let mut prime_factors = vec![];
     while n > 1 {
        let i = (2..n+1).find(|x| n % x == 0 ).unwrap(); 
        prime_factors.push(i);
        n /= i;
    } 

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    prime_factors

}