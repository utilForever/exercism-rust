pub fn reverse(input: &str) -> String {
    let mut ret = String::new();
    input.chars().rev().for_each(|c| ret.push(c));
    ret
}
