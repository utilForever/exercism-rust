pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    
    let mut cnt = 0;
    
    while n > 1 {
        match n % 2 {
            0 => {
                n /= 2;
            },
            1 => {
                if n >= u64::MAX / 3 {
                    return None;
                }
                
                n = n * 3 + 1;
            },
            _ => {
                return None
            },
        }

        cnt += 1;
    }

    Some(cnt)
}
