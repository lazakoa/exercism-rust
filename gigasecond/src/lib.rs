use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let base: i64 = 10;
    let gigasecond: i64 = base.pow(9);
    let end = start + Duration::seconds(gigasecond);
    return end;
}
