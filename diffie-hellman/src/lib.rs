use rand;

pub fn private_key(p: u64) -> u64 {
    // "Pick a private key greater than 1 and less than {p}"
    2 + rand::random::<u64>() % (p-2)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // "Calculate public key using prime numbers {p} and {g}, and private key {a}"
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // "Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}"
    mod_exp(b_pub, a, p)
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    // modular exponentiation function to compute (base^exp) mod modulus
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}
