use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Self { hours, minutes };
        clock.normalize();

        clock
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.normalize();

        self.clone()
    }

    fn normalize(&mut self) {
        if self.minutes < 0 {
            while self.minutes < 0 {
                self.minutes += 60;
                self.hours -= 1;
            }
        } else if self.minutes >= 60 {
            while self.minutes >= 60 {
                self.minutes -= 60;
                self.hours += 1;
            }
        }

        if self.hours < 0 {
            while self.hours < 0 {
                self.hours += 24;
            }
        } else if self.hours >= 24 {
            while self.hours >= 24 {
                self.hours -= 24;
            }
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
