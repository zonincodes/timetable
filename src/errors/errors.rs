#[derive(PartialEq, Debug)]
pub enum CustomParseError {
    ParseInt,
    ParseTime(ParseTime)
}

#[derive(PartialEq, Debug)]
pub enum ParseTime {
    ParseLocale,
    ParseDay,
    ParseHour(String),
    ParseMin(String),
}