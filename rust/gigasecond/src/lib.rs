use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    println!("{:?}", start);
    let new_date = start + Duration::seconds(1000000000);
    new_date
}
