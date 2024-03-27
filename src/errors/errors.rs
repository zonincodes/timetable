/// Error
#[derive(PartialEq, Debug)]
pub enum TimeTableError {
    ParseInt,
    ParseTime(ParseTime),
}


/// Errors for time parsing
#[derive(PartialEq, Debug)]
pub enum ParseTime {
    ParseLocale,
    ParseDay,
    ParseHour(String),
    ParseMin(String),
}
