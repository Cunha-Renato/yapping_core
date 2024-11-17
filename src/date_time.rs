use std::fmt::Display;
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    date: Date,
    time: Time,
}
impl DateTime {
    pub fn from_utc(date_time: &chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            date: Date::new(date_time.day(), date_time.month(), date_time.year() as u32).unwrap(),
            time: Time::new(date_time.hour(), date_time.minute(), date_time.second()).unwrap(),
        }
    }
    
    pub fn to_utc(&self) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>> {
        Ok(
            chrono::NaiveDate::from_ymd_opt(self.date.year() as i32, self.date.month(), self.date.day()).ok_or("Date is not supported!")?
                .and_hms_opt(self.time.hour(), self.time.minute(), self.time.second()).ok_or("Time is not supported")?
                .and_utc()
        )
    }
    
    pub fn to_local(&self) -> Result<chrono::DateTime<chrono::Local>, Box<dyn std::error::Error>> {
        Ok(self.to_utc()?.with_timezone(&chrono::Local))
    }
}
impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date: {}, Time: {}", self.date, self.time)
    }
}

const DAY_MASK: u32 =       0b00000000000000000000000000011111;
const MONTH_MASK: u32 =     0b00000000000000000000000111100000;
const YEAR_MASK: u32 =      0b11111111111111111111111000000000;
const SECOND_MASK: u32 =    0b00000000000000000000000000111111;
const MINUTE_MASK: u32 =    0b00000000000000000000111111000000;
const HOUR_MASK: u32 =      0b00000000000000011111000000000000;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Date {
    date: u32,    
}
impl Date {
    pub fn new(day: u32, month: u32, year: u32) -> Result<Self, Box<dyn std::error::Error>> {
        if year > YEAR_MASK
            || (month < 1 || month > 12)
            || (day < 1 || day > 31)
        {
            return Err("Invalid date format!".into())
        }
        
        Ok(Self {
            date: (year << 9) | (month << 5) | day
        })
    }

    pub const fn day(&self) -> u32 {
        self.date & DAY_MASK
    }
    
    pub const fn month(&self) -> u32 {
        (self.date & MONTH_MASK) >> 5
    }
    
    pub const fn year(&self) -> u32 {
        (self.date & YEAR_MASK) >> 9
    }
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.day(), self.month(), self.year())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Time {
    time: u32
}
impl Time {
    pub fn new(hour: u32, minute: u32, second: u32) -> Result<Self, Box<dyn std::error::Error>> {
        if hour > 23
            || minute > 59
            || second > 59
        {
            return Err("Invalid time format!".into())
        }
        
        Ok(Self {
            time: (hour << 12) | (minute << 6) | second
        })
    }

    pub const fn second(&self) -> u32 {
        self.time & SECOND_MASK
    }
    
    pub const fn minute(&self) -> u32 {
        (self.time & MINUTE_MASK) >> 6
    }
    
    pub const fn hour(&self) -> u32 {
        (self.time & HOUR_MASK) >> 12
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour(), self.minute(), self.second())
    }
}