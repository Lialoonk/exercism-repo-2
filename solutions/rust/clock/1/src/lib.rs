use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized_minutes = total_minutes % (24 * 60);
        let normalized_minutes = if normalized_minutes < 0 {
            normalized_minutes + 24 * 60
        } else {
            normalized_minutes
        };
        
        let hours = normalized_minutes / 60;
        let minutes = normalized_minutes % 60;
        
        Clock { hours, minutes }
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn subtract_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes - minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}