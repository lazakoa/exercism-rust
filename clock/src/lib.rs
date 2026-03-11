use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_total = minutes.rem_euclid(60);
        let hours_total = (hours + minutes / 60).rem_euclid(24);

        let mut modifier = 0;
        if minutes < 0 {
            if minutes_total != 0 {
                modifier = -1;
            }
        }
        
        return Clock {
            hours: hours_total + modifier,
            minutes: minutes_total,
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_added = self.minutes + minutes;
        let minutes_total = self.hours * 60 + minutes_added;
        let hours_total = minutes_total / 60;
        
        let mut modifier = 0;
        if minutes_total < 0 {
           modifier = -1; 
        }

        println!("{}", hours_total);
 
        return Clock {
            hours: (modifier + hours_total).rem_euclid(24),
            minutes: (minutes_total - hours_total * 60).rem_euclid(60),
        };
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        return self.hours == other.hours && self.minutes == other.minutes;
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}