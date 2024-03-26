use crate::functions::functions::get_input;

pub enum Validate {
    Day,
    Title,
    Time,
    Locale,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub day: String,
    pub title: String,
    pub start: String,
    pub start_locale: String,
    pub end: String,
    pub end_locale: String,
    pub is_valid: bool,
}

impl Event {
    // Takes user inputs and returns a vector
    pub fn get_event_shedule(&mut self) {
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

    pub fn new() -> Self {
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

    pub fn validate(&mut self) {
        self.is_valid = true;
    }
}


#[derive(PartialEq, Debug)]
pub enum CustomError {
    ParseInt
}
