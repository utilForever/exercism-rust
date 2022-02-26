use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    use time::Duration;
    
    start + Duration::seconds(1_000_000_000)
}
