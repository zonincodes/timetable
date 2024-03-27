// Get input from the user and sanitizes

use std::io;

use crate::{
    objects::objects::{Event, Validate},
    parsing::parsing::{_parse_time, _parse_title},
};

pub fn get_input(detail: &str, validate: Validate) -> String {
    loop {
        let mut input: String = String::new();

        println!("{detail}");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_string();
        match validate {
            Validate::Day => {
                if _parse_title(&input) {
                    break input;
                } else {
                    println!("Wrong Day, enter correct day");
                    continue;
                }
            }
           
            Validate::Title => {
                if _parse_title(&input) {
                    break input;
                } else {
                    println!("Title Length too short");
                    continue;
                }
            }

            Validate::Time => {
                let time = _parse_time(&input);
                match time {
                    Ok(_) => break input,
                    Err(_) => {
                        println!("That date ain't right");
                        continue;
                    }
                }
            }
        }
    }
}



// Takes user inputs and modifys or creates new event
fn _modify() {}

// print_days of week
fn print_days_of_week() {
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
}

// Formats the output of the timetable
pub fn print_format(vec: &Vec<[&Event; 8]>) {
    let string: &str = "-";
    print_days_of_week();
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


// Returns the time of day for particular event
fn time_of_day(x: usize) -> String {
    let time_day: [&str; 11] = [
        "07:00am ", "08:00am ", "09:00am ", "10:00am ", "11:00am ", "12:00pm ", "13:00pm ",
        "14:00pm ", "15:00pm ", "16:00pm ", "17:00pm ",
    ];

    time_day[x].to_string()
}

#[cfg(test)]
mod test {
    use std::io::{self, Cursor};


    #[test]
    #[ignore] // test doesnt work
    fn test_get_input() {
        let _mock_input = Cursor::new("pm".as_bytes());
        let mut mock_stdin = io::stdin();

        // swap the actual stdin with mock input
        let _ = std::mem::swap(&mut io::stdin(), &mut mock_stdin);


        // restore the actual stdIn
        let _ = std::mem::swap(&mut io::stdin(), &mut mock_stdin);
    }
}
