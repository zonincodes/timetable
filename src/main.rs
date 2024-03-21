use std::{fmt::Error, io, vec};

fn main() {
    let time_table: Vec<[[&str; 5]; 8]> = vec![[["  "; 5]; 8]; 7];
    print_instructions();
    get_event_shedule();
    print_format(&time_table)
}

// Takes user inputs and returns a vector
fn get_event_shedule() {
    // Vector containing an event
    let mut event_details: Vec<String> = vec![];

    let input: String = get_input("Enter Event Title: More than 3 Letters");
    event_details.push(input);

    let input: String = get_input("Enter Start time! format: 1:20");
    event_details.push(input);

    let input: String = get_input("Enter Locale: am/pm");
    event_details.push(input);

    let input: String = get_input("Enter End time! format: 1:20");
    event_details.push(input);

    let input: String = get_input("Enter Locale `pm`");
    event_details.push(input);

    println!("{:#?}", event_details)
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
fn _parse_time(_event: Vec<String>) -> Vec<u32> {
    todo!()
}

fn _parse_locale(locale: &String) -> bool {
    if locale == "pm" || locale == "am" {
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
}

// Takes user inputs and returns a vector
fn _user_inputs(_vec: &Vec<[[&str; 5]; 8]>) -> Result<Vec<String>, Error> {
    todo!()
}

// Takes user inputs and modifys or creates new event
fn _modify() {}

// Formats the output of the timetable
fn print_format(vec: &Vec<[[&str; 5]; 8]>) {
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
                    let mut my_str: String = vec[x][i][z].to_string();
                    if my_str.len() > 7 {
                        my_str = format!("{}...", &my_str[..7])
                    }
                    let str = format!("{str:>10}", str = my_str);
                    string.push_str(&str);
                    string.push_str(" | ");
                } else if z == 1 {
                    let str: String = format!("{str:>8}{b}", str = vec[x][i][z], b = vec[x][i][z + 1]);
                    string.push_str(&str);
                    string.push_str(" | ");
                } else {
                    let str: String = format!("{str:>8}{b}", str = vec[x][i][z + 1], b = vec[x][i][z + 2]);
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
