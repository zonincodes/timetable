use std::io;

use crate::{functions::functions::print_format, objects::objects::Event};


/// Runs the program
pub fn run() {
    print_instructions();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        if input.trim().to_lowercase() == "q" {
            break;
        }

        let input = input.trim().parse::<i32>();
        match input {
            Ok(num) => {
                _handle_user_option(&num);
            }
            Err(_) => {
                println!("Unknown Command");
                continue;
            }
        };
    }
}

fn _shedule_event() {
    let mut event: Event = Event::new();
    event.get_event_shedule();
}

fn _handle_user_option(input: &i32) {
    let event = Event::new();
    let time_table: Vec<[&Event; 8]> = vec![[&event; 8]; 7];
    match input {
        1 => {
            println!("{:*>30}", "*");
            println!("Time-Table");
            println!("{:*>30}", "*");
            print_format(&time_table);
            print_instructions();
        }
        2 => {
            println!("{:->30}", "-");
            println!("Shedule A New Event");
            println!("{:->30}", "-");
            _shedule_event();
            print_instructions();
        }

        3 => {
            println!("{:->30}", "-");
            println!("Coming Soon");
            println!("{:->30}", "-");
            print_instructions();
        }
        4 => {
            println!("{:->30}", "-");
            println!("Coming Soon");
            println!("{:->30}", "-");
            print_instructions();
        }
        5 => print_instructions(),

        6 => print_help(),
        _ => {
            println!("{:->30}", "-");
            println!("Unknown Command");
            println!("{:->30}", "-");
            print_instructions();
        }
    }
}

fn print_help() {
    println!("
        {:*>30}", "*");
    println!("
                   HELP");
    println!("
        {:->30}", "-");

    println!("
        --days of week\n
            --Week Days
                Sunday Monday Tuesday Wednesday
                Thursday Friday and Suturday\n
            --Start day 
                Sunday\n
            --End day
                Saturday\n
        --hours
            Enter start time => 01:00AM  \n
            Enter end time =>   02:00AM \n
            Note Event Should not exceed 1hr \n
    ");


}

pub fn print_instructions() {
    println!("
        {:*>30}", "*");
    println!("
                   MENU");
    println!("
              Choose option");

    println!("
        {:*>30}", "*");
    println!("
        1>> Show Time Table");
    println!("
        2>> Schedule Event");
    println!("
        3>> Update Event");
    println!("
        4>> Delete Event");
    println!("
        5>> Print Instructions");
    println!("
        6>> Help");
    println!("
        Q>> Quit");
}
