pub fn raindrops(n: u32) -> String {
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string();
    }
    
    let mut ret = String::new();

    if n % 3 == 0 {
        ret += &String::from("Pling");
    }
    if n % 5 == 0 {
        ret += &String::from("Plang");
    }
    if n % 7 == 0 {
        ret += &String::from("Plong");
    }

    ret
}
