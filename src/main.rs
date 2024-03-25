use std::{io, vec};

enum Validate {
    Day,
    Title,
    Time,
    Locale,
}

#[derive(Debug, Clone)]
pub struct Event {
    day: String,
    title: String,
    start: String,
    start_locale: String,
    end: String,
    end_locale: String,
    is_valid: bool,
}

impl Event {
    // Takes user inputs and returns a vector
    fn get_event_shedule(&mut self) {
        // Vector containing an event
        let input: String = get_input("Enter Day Of Week: More than 3 Letters", Validate::Day);
        self.day = input;
        let input: String = get_input("Enter Event Title: More than 3 Letters", Validate::Title);
        self.title = input;
        let input: String = get_input("Enter Start time! format: 1:00", Validate::Time);
        self.start = input;
        let input: String = get_input("Enter Locale: am/pm", Validate::Locale);
        self.start_locale = input;
        let input: String = get_input("Enter End time! format: 2:00", Validate::Time);
        self.end = input;
        let input: String = get_input("Enter Locale pm", Validate::Locale);
        self.end_locale = input;
    }

    fn new() -> Self {
        Event {
            day: "  ".to_string(),
            title: "foxyLowProxy".to_string(),
            start: "  ".to_string(),
            start_locale: "  ".to_string(),
            end: "  ".to_string(),
            end_locale: "  ".to_string(),
            is_valid: false,
        }
    }

    fn validate(&mut self) {
        self.is_valid = true;
    }
}

fn _validate_event(_event: &Event) -> bool {
    if !_parse_locale(&_event.end_locale) || !_parse_locale(&_event.start_locale) {
        return false;
    }

    // if
    todo!()
}



fn _parse_start_end(event: &Event) -> bool {
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

fn calculate_min_into_day(time: &Vec<u32>) -> u32 {
    time[0] * 60 + time[1]
}

fn _parse_time<'a>(time: &String) -> Option<Vec<u32>> {
    let time: Vec<&str> = time.split(":").to_owned().collect();
    let time: Vec<u32> = time
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    Some(time)
}

fn _parse_locale(locale: &String) -> bool {
    if locale.to_lowercase() == "pm" || locale.to_lowercase() == "am" {
        true
    } else {
        false
    }
}

fn _parse_title(title: &String) -> bool {
    if title == "" || title.len() < 3 {
        false
    } else {
        true
    }
}

// Get input from the user and sanitizes
fn get_input(detail: &str, validate: Validate) -> String {
    loop {
        let mut input: String = String::new();

        println!("{detail}");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match validate {
            Validate::Day => {
                if _parse_title(&input) {
                    break input.trim().to_string();
                } else {
                    println!("Wrong Day, enter correct day");
                    continue;
                }
            }
            Validate::Locale => {
                if _parse_locale(&input) {
                    break input.trim().to_string();
                } else {
                    println!("Wrong Locale, enter correctly AM/PM");
                    continue;
                }
            }

            Validate::Title => {
                if _parse_title(&input) {
                    break input.trim().to_string();
                } else {
                    println!("Title Length too short");
                    continue;
                }
            }

            Validate::Time => {
                let time = _parse_time(&input);
                match time {
                    Some(_) => break input.trim().to_string(),
                    None => {
                        println!("That date ain't right");
                    continue;
                    },
                }
            }
        }

        // break input.trim().to_string();
    }
}

// Prints the instructions to the console
fn print_instructions() {
    println!("## MENU ##");
    println!("Choose option");
    println!("1>> Show Time Table");
    println!("2>> Schedule Event");
    println!("3>> Update Event");
    println!("4>> Delete Event");
    println!("5>> Print Instructions");
}

// Takes user inputs and modifys or creates new event
fn _modify() {}

// Formats the output of the timetable
fn print_format(vec: &Vec<[&Event; 8]>) {
    let string: &str = "-";
    println!("{string:->100}");

    let days_of_week: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    println!(
        "{time:>7} | {sun:>9} | {mon:>10} | {tue:>10} | {wen:>10} | {thur:>10} | {fri:>10} | {sat:>10} |",
        time = "|  Time",
        sun = days_of_week[0],
        mon = days_of_week[1],
        tue = days_of_week[2],
        wen = days_of_week[3],
        thur = days_of_week[4],
        fri = days_of_week[5],
        sat = days_of_week[6],
    );
    println!("{string:->100}");

    for i in 0..8 {
        for z in 0..3 {
            let mut string: String = String::from("");
            if z == 0 {
                string.push_str(&time_of_day(i));
            } else {
                string.push_str("|       ");
            }
            for x in 0..7 {
                if x == 0 {
                    string.push_str("|")
                }
                if z == 0 {
                    if vec[x][i].title.len() > 7 {
                        let my_str = format!("{}...", &vec[x][i].title[..7]);
                        let str = format!("{str:>10}", str = my_str);
                        string.push_str(&str);
                        string.push_str(" | ");
                    } else {
                        let str = format!("{str:>10}", str = &vec[x][i].title);
                        string.push_str(&str);
                        string.push_str(" | ");
                    }
                } else if z == 1 {
                    let str: String = format!(
                        "{str:>8}{b}",
                        str = vec[x][i].start,
                        b = vec[x][i].start_locale
                    );
                    string.push_str(&str);
                    string.push_str(" | ");
                } else {
                    let str: String =
                        format!("{str:>8}{b}", str = vec[x][i].end, b = vec[x][i].end_locale);
                    string.push_str(&str);
                    string.push_str(" | ");
                }
            }
            println!("{}", string);
        }
        println!("{string:->100}");
    }
}

fn time_of_day(x: usize) -> String {
    let time_day: [&str; 11] = [
        "07:00am ", "08:00am ", "09:00am ", "10:00am ", "11:00am ", "12:00pm ", "13:00pm ",
        "14:00pm ", "15:00pm ", "16:00pm ", "17:00pm ",
    ];

    time_day[x].to_string()
}


fn main() {
    print_instructions();
    // get_event_shedule();
    let event = Event::new();
    let time_table: Vec<[&Event; 8]> = vec![[&event; 8]; 7];

    print_format(&time_table)
}

#[cfg(test)]
mod test {
    use crate::{_parse_locale, _parse_time, _parse_title};

    #[test]
    fn parse_time_test() {
        let time = String::from("01:30");

        assert_eq!(_parse_time(&time), Some(vec![1, 30]))
    }

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
}
