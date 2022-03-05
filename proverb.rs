pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    
    let mut ret = String::new();
    
    for i in 0..list.len() - 1 {
        ret += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).to_string();
    }
    ret += &format!("And all for the want of a {}.", list[0]).to_string();

    ret
}
