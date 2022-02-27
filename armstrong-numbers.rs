pub fn is_armstrong_number(num: u32) -> bool {
    let mut new_num = num;
    let mut digit = 0;
    let mut nums = Vec::new();
    
    while new_num >= 10 {
        nums.push(new_num % 10);
        digit += 1;

        new_num /= 10;
    }

    nums.push(new_num);
    digit += 1;

    let sum: u32 = nums.iter().map(|val| val.pow(digit)).sum();
    if sum == num { true } else { false }
}
