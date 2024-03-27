use crate::{
    errors::errors::{CustomParseError, ParseTime},
    objects::objects::Event
};

pub fn _parse_start_end(event: &Event) -> bool {
    let start = _parse_time(&event.start).unwrap();
    let end = _parse_time(&event.end).unwrap();

    if event.start_locale.to_lowercase() == "pm" && event.end_locale.to_lowercase() == "am" {
        return false;
    }

    let start_time_in_min = calculate_min_into_day(&start);
    let end_time_in_min = calculate_min_into_day(&end);
    if (end_time_in_min - start_time_in_min) > 60 {
        return false;
    }

    true
    // todo!()
}

pub fn calculate_min_into_day(time: &Vec<i32>) -> i32 {
    time[0] * 60 + time[1]
}

// Check if the user input for time ia velid
pub fn _parse_time<'a>(time: &String) -> Result<Vec<i32>, CustomParseError> {
    if time.len() != 7 {
        return Err(CustomParseError::ParseInt);
    }
    let local_time = &time[..5];
    println!("{:#?}", local_time);
    let local_time: Vec<&str> = local_time.split(":").to_owned().collect();

    let hour: i32 = match local_time[0].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            return Err(CustomParseError::ParseTime(ParseTime::ParseHour(
                String::from("Wrong hour input"),
            )))
        }
    };

    let min: i32 = match local_time[1].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            return Err(CustomParseError::ParseTime(ParseTime::ParseMin(
                "Wrong min".into(),
            )))
        }
    };
    let locale = &time[5..].into();
    println!("Locale {}", locale);
    if !_parse_locale(&locale) {
        return Err(CustomParseError::ParseTime(ParseTime::ParseLocale));
    };

    if hour < 13 && min < 60 {

        Ok(vec![hour, min])
    } else {
        Err(CustomParseError::ParseInt)
    }
}

pub fn _parse_locale(locale: &String) -> bool {
    if locale.to_lowercase() == "pm".to_string() || locale.to_lowercase() == "am".to_string() {
        true
    } else {
        false
    }
}

pub fn _parse_title(title: &String) -> bool {
    if title == "" || title.len() < 3 {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::parsing::parsing::{_parse_locale, _parse_time, _parse_title};

    #[test]
    fn test_locale() {
        let locale = String::from("am");
        let locale_1 = String::from("PM");

        assert_eq!(true, _parse_locale(&locale));
        assert_eq!(true, _parse_locale(&locale_1));
    }

    #[test]
    fn test_title() {
        let title: String = "".to_string();
        let title_1: String = "Hello".to_string();
        let title_2: String = "Wue".to_string();

        assert_eq!(false, _parse_title(&title));
        assert_eq!(true, _parse_title(&title_1));
        assert_eq!(true, _parse_title(&title_2));
    }

    #[test]
    fn parse_time_test() {
        let time = String::from("01:30PM");

        assert_eq!(_parse_time(&time), Ok(vec![1, 30]))
    }
}
