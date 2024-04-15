use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)   // *1
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clock {{ hours: {}, minutes: {} }}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}


impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        // Normalize the time
        hours += minutes / 60;
        minutes %= 60;
        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        hours %= 24;
        if hours < 0 {
            hours += 24;
        }

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
