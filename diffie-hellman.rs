pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_mod(b_pub, a, p)
}

// Reference: https://en.wikipedia.org/wiki/Modular_exponentiation
pub fn exp_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result: u64 = 1;
    base = base % modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        exponent = exponent / 2;
        base = (base * base) % modulus;
    }

    result
}
