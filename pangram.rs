/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet = [false; 26];

    for c in sentence.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            alphabet[c as usize - 'a' as usize] = true;
        }
    }
    
    alphabet.iter().all(|&x| x)
}
