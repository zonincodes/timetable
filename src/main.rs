use std::{fmt::format, vec};

fn main() {
    let mut time_table: Vec<[[&str; 5]; 8]> = vec![
        [
            ["JabaWalkers", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
           
        ],
        [
            ["JasonBing", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
        ],
        [
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
        ],
        [
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
        ],
        [
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
        ],
        [
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
        ],
        [
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  "],
        ],
    ];

    // Print Data

    // for (idx, day) in time_table.iter().enumerate() {
    //     print_day(idx);
    //     for event in day {
    //         println!(
    //             "Title: {} \nStart: {}{} \nEnd{}{} \n",
    //             event[0], event[1], event[2], event[3], event[4]
    //         );
    //     }
    // }

    print_format(&time_table)
}

fn print_format(vec: &Vec<[[&str; 5]; 8]>) {
    let string = "-";
    println!("{string:->100}");

    let days_of_week = [
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
    // println!(
    //     "{title:>6} \n{start}{locale_start} \n{end}{locale_end}",
    //     title = vec[i][i][0],
    //     start = vec[i][i][1],
    //     locale_start = vec[i][i][2],
    //     end = vec[i][i][3],
    //     locale_end = vec[i][i][4],
    // );
    // println!("{string:->79}");

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
                    let mut  my_str =  vec[x][i][z].to_string();
                    if my_str.len() > 7 {
                        my_str = format!("{}...", &my_str[..7])
                    }
                    let str = format!("{str:>10}", str = my_str);
                    string.push_str(&str);
                    string.push_str(" | ");
                } else if z == 1 {
                    let str = format!("{str:>8}{b}", str = vec[x][i][z], b = vec[x][i][z + 1]);
                    string.push_str(&str);
                    string.push_str(" | ");
                } else {
                    let str = format!("{str:>8}{b}", str = vec[x][i][z + 1], b = vec[x][i][z + 2]);
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
    let time_day = [
        "07:00am ", "08:00am ", "09:00am ", "10:00am ", "11:00am ", "12:00pm ", "13:00pm ",
        "14:00pm ", "15:00pm ", "16:00pm ", "17:00pm ",
    ];

    time_day[x].to_string()
}
