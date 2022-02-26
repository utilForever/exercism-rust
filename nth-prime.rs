fn is_prime(n: u32) -> bool {
    if n < 2 {
        return true;
    }

    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut idx = 0;
    let mut num = 2;

    loop {
        if is_prime(num) {
            idx += 1;
        }

        if idx == n + 1 {
            break;
        }

        num += 1;
    }

    num
}
