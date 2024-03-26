// Prints the instructions to the console
use timetable::{functions::functions::{print_format, print_instructions}, objects::objects::Event};

fn main() {
    print_instructions();
    // get_event_shedule();
    let mut event = Event::new();
    event.get_event_shedule();
    let time_table: Vec<[&Event; 8]> = vec![[&event; 8]; 7];

    print_format(&time_table);
}


   

    

    
 
