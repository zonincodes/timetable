use crate::{functions::functions::get_input, parsing::parsing::parse_start_end};

/// Validates Sections of the input
pub enum Validate {
    Day,
    Title,
    Time,
}

///# `Event` store the event
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
    /// Edits a bare metal event and updates the fields from user
    pub fn get_event_shedule(&mut self) {
        // Get day input `["Sunday", "Monday", ...]`
        let input: String = get_input(
            "Enter Day Of Week: `Sunday/Monday/.../..` Letters",
            Validate::Day,
        );
        self.day = input;
        // Gets the `title for event`
        let input: String = get_input("Enter Event Title: More than 3 Letters", Validate::Title);
        self.title = input;
        // Gets the `start time` for event`
        let input: String = get_input("Enter Start time! format: 01:00PM", Validate::Time);
        self.start = input;
        // Gets the `end time` for event`
        let input: String = get_input("Enter End time! format: 02:00PM", Validate::Time);
        self.end = input;
    }

    /// Creates an empty event
    pub fn new() -> Self {
        Event {
            day: "  ".to_string(),
            title: "  ".to_string(),
            start: "  ".to_string(),
            start_locale: "  ".to_string(),
            end: "  ".to_string(),
            end_locale: "  ".to_string(),
            is_valid: false,
        }
    }

    /// Validate's if the event is correct
    pub fn validate(&mut self) {
        if parse_start_end(&self) {
            self.is_valid = true;
        }
    }
}
