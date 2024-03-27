use std::io;

use crate::{functions::functions::print_format, objects::objects::Event};

pub fn run() {
    print_instructions();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        if input.trim().to_lowercase() == "q" {
            break;
        }

        let input = input.trim().parse::<i32>().unwrap();
        _handle_user_option(&input);

    }
}

fn _shedule_event(){
    let mut event: Event = Event::new();
    event.get_event_shedule(); 
}

fn _handle_user_option(input: &i32) {
    let event = Event::new();
    let time_table: Vec<[&Event; 8]> = vec![[&event; 8]; 7];
    match input {
        1 => print_format(&time_table),
        2 => _shedule_event(),
        5 => print_instructions(),
        _ => (),
    }
}

pub fn print_instructions() {
    println!("## MENU ##");
    println!("Choose option");
    println!("1>> Show Time Table");
    println!("2>> Schedule Event");
    println!("3>> Update Event");
    println!("4>> Delete Event");
    println!("5>> Print Instructions");
}
