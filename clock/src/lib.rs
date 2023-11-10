use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {minutes: hours * 60 + minutes}.handle_overflow()
    }

    fn handle_overflow(&self) -> Self {
        // rendi i valori positivi modulo 24h e i negativi positivi modulo 24h
        let new_minutes = if self.minutes >= 0 { self.minutes % (60*24) } else { self.minutes % (60*24) + 60*24 } ;
        Clock {minutes: new_minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { minutes: self.minutes + minutes}.handle_overflow()
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.minutes / 60 , self.minutes % 60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Add<i32> for Clock {
    type Output = Self;

    fn add(self, minutes: i32) -> Self {
        Clock { minutes: self.minutes + minutes }.handle_overflow()
    }
}

impl Sub<i32> for Clock {
    type Output = Self;

    fn sub(self, minutes: i32) -> Self {
        Clock { minutes: self.minutes - minutes }.handle_overflow()
    }
}

impl Add<Clock> for Clock {
    type Output = Self;

    fn add(self, other: Clock) -> Self {
        Clock { minutes: self.minutes + other.minutes }.handle_overflow()
    }
}

impl Sub<Clock> for Clock {
    type Output = Self;

    fn sub(self, other: Clock) -> Self {
        Clock { minutes: self.minutes - other.minutes }.handle_overflow()
    }
}