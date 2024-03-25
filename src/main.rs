use std::{io, vec};

fn main() {
    print_instructions();
    // get_event_shedule();
    let event = Event::new();
    let time_table: Vec<[&Event; 8]> = vec![[&event; 8]; 7];

    print_format(&time_table)
}

#[derive(Debug, Clone)]
pub struct Event {
    day: String,
    title: String,
    start: String,
    start_locale: String,
    end: String,
    end_locale: String,
}

impl Event {
    // Takes user inputs and returns a vector
    fn get_event_shedule(&mut self) {
        // Vector containing an event
        let input: String = get_input("Enter Day Of Week: More than 3 Letters");
        self.day = input;
        let input: String = get_input("Enter Event Title: More than 3 Letters");
        self.title = input;
        let input: String = get_input("Enter Start time! format: 1:20");
        self.start = input;
        let input: String = get_input("Enter Locale: am/pm");
        self.start_locale = input;
        let input: String = get_input("Enter End time! format: 1:20");
        self.end = input;
        let input: String = get_input("Enter Locale `pm`");
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
        }
    }
}

fn _validate_event(_event: &Vec<String>) -> bool {
    todo!()
}

fn _parse_start_end(
    _start: &Vec<u32>,
    _end: &Vec<u32>,
    _start_locale: &String,
    _end_locale: &String,
) -> bool {
    todo!()
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
fn get_input(detail: &str) -> String {
    let mut input: String = String::new();
    println!("{detail}");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
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
