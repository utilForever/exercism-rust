pub fn factors(mut n: u64) -> Vec<u64> {
    let mut ret = Vec::new();

    if n == 1 {
        return ret;
    }

    let mut i = 2;
    loop {
        if n % i == 0 {
            ret.push(i);
            n /= i;
        } else {
            i += 1;
        }

        if i > n {
            break;
        }
    }

    ret
}
