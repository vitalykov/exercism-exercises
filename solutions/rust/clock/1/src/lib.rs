use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn normalize(hours: i32, minutes: i32) -> (i32, i32) {
    let actual_minutes = (minutes % 60 + 60) % 60;
    let add_hours = (minutes - actual_minutes) / 60;
    let actual_hours = ((hours + add_hours) % 24 + 24) % 24;
    (actual_hours, actual_minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from(normalize(hours, minutes))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from(normalize(self.hours, self.minutes + minutes))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<(i32, i32)> for Clock {
    fn from((hours, minutes): (i32, i32)) -> Self {
        Self { hours, minutes }
    }
}
