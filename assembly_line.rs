pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base_production_rate = 221.0;
    
    match speed as i32 {
        0 => 0.0,
        1..=4 => speed as f64 * base_production_rate,
        5..=8 => speed as f64 * base_production_rate * 0.9,
        9..=10 => speed as f64 * base_production_rate * 0.77,
        _ => panic!("Invalid speed"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
