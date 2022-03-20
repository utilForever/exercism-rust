pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    } else if len > digits.len() {
        return Vec::new();
    }

    let mut result = Vec::new();
    
    for i in 0..=digits.len() - len {
        result.push(digits[i..i + len].to_string());
    }
    
    result
}
