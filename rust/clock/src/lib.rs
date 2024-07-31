use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        if minutes < 0 {
            // get number of hours with 60/mins and * by -1 to get pos number to subtract
            println!("{}", minutes as f32 / 60.0);
            println!("{},{}", minutes / 60 * -1, 60 + minutes % 60);
            hours -= 60 / minutes * -1;
            minutes = 60 + minutes % 60;
        }
        if hours < 0 {
            hours = 24 + hours % 24;
        }
        if minutes % 60 == 0 {
            hours += minutes / 60;
            minutes = 0;
        }
        if minutes >= 60 {
            hours += minutes / 60;
            minutes = minutes % 60;
        }
        if hours > 24 {
            hours = hours % 24;
        }
        if hours % 24 == 0 {
            hours = 0;
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_hours: String = self.hours.to_string();
        let mut string_minutes: String = self.minutes.to_string();

        if self.hours.to_string().chars().count() == 1 {
            string_hours = "0".to_owned() + &self.hours.to_string();
        }
        if self.minutes.to_string().chars().count() == 1 {
            string_minutes = "0".to_owned() + &self.minutes.to_string();
        }

        write!(f, "{}:{}", string_hours, string_minutes)
    }
}
