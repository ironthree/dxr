#![allow(missing_docs)]

use std::fmt::Display;
use std::str::FromStr;

use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take;
use winnow::Parser;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,

    hour: u8,
    minute: u8,
    second: u8,
}

impl DateTime {
    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}{:02}{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second,
        )
    }
}

impl FromStr for DateTime {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTimeParser
            .parse(s)
            .map_err(|e| DateTimeParseError::InvalidFormat(e.to_string()))
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum DateTimeParseError {
    #[error("Invalid format for dateTime.iso8601 value: {}", .0)]
    InvalidFormat(String),
    #[error("Invalid dateTime.iso8601 value: Month out of range ({})", .0)]
    InvalidMonth(u8),
    #[error("Invalid dateTime.iso8601 value: Day out of range ({})", .0)]
    InvalidDay(u8),
    #[error("Invalid dateTime.iso8601 value: Hour out of range ({})", .0)]
    InvalidHour(u8),
    #[error("Invalid dateTime.iso8601 value: Minutes out of range ({})", .0)]
    InvalidMinutes(u8),
    #[error("Invalid dateTime.iso8601 value: Seconds out of range ({})", .0)]
    InvalidSeconds(u8),
}

struct DateTimeParser;

impl Parser<&str, DateTime, ContextError> for DateTimeParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<DateTime> {
        let year = YearParser.parse_next(input)?;
        let month = MonthParser.parse_next(input)?;
        let day = DayParser {
            max: length_of_month(year, month),
        }
        .parse_next(input)?;

        _ = parse_datetime_sep(input)?;

        let hour = HourParser.parse_next(input)?;
        _ = parse_time_sep(input)?;
        let minute = MinuteParser.parse_next(input)?;
        _ = parse_time_sep(input)?;
        let second = SecondParser.parse_next(input)?;

        Ok(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

struct YearParser;

impl Parser<&str, u16, ContextError> for YearParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<u16> {
        take(4usize)
            .parse_to()
            .context(StrContext::Label("year"))
            .parse_next(input)
    }
}

struct MonthParser;

impl Parser<&str, u8, ContextError> for MonthParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<u8> {
        take(2usize)
            .parse_to()
            .try_map(|month| match month {
                1..=12 => Ok(month),
                _ => Err(DateTimeParseError::InvalidMonth(month)),
            })
            .context(StrContext::Label("month"))
            .parse_next(input)
    }
}

struct DayParser {
    max: u8,
}

impl Parser<&str, u8, ContextError> for DayParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<u8> {
        take(2usize)
            .parse_to()
            .try_map(|day| {
                if day == 0 || day > self.max {
                    Err(DateTimeParseError::InvalidDay(day))
                } else {
                    Ok(day)
                }
            })
            .context(StrContext::Label("day"))
            .parse_next(input)
    }
}

struct HourParser;

impl Parser<&str, u8, ContextError> for HourParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<u8> {
        take(2usize)
            .parse_to()
            .try_map(|hour| {
                if hour > 24 {
                    Err(DateTimeParseError::InvalidHour(hour))
                } else {
                    Ok(hour)
                }
            })
            .context(StrContext::Label("hour"))
            .parse_next(input)
    }
}

struct MinuteParser;

impl Parser<&str, u8, ContextError> for MinuteParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<u8> {
        take(2usize)
            .parse_to()
            .try_map(|minute| {
                if minute > 59 {
                    Err(DateTimeParseError::InvalidHour(minute))
                } else {
                    Ok(minute)
                }
            })
            .context(StrContext::Label("minute"))
            .parse_next(input)
    }
}

struct SecondParser;

impl Parser<&str, u8, ContextError> for SecondParser {
    fn parse_next(&mut self, input: &mut &str) -> winnow::Result<u8> {
        take(2usize)
            .parse_to()
            .try_map(|second| {
                if second > 59 {
                    Err(DateTimeParseError::InvalidHour(second))
                } else {
                    Ok(second)
                }
            })
            .context(StrContext::Label("second"))
            .parse_next(input)
    }
}

fn parse_datetime_sep(input: &mut &str) -> winnow::Result<char> {
    'T'.context(StrContext::Label("T separator"))
        .context(StrContext::Expected(StrContextValue::CharLiteral('T')))
        .parse_next(input)
}

fn parse_time_sep(input: &mut &str) -> winnow::Result<char> {
    ':'.context(StrContext::Label(": separator"))
        .context(StrContext::Expected(StrContextValue::CharLiteral(':')))
        .parse_next(input)
}

fn length_of_month(year: u16, month: u8) -> u8 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!(),
    }
}

#[allow(clippy::needless_bool)]
fn is_leap_year(year: u16) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveDateTime> for DateTime {
    fn from(value: chrono::NaiveDateTime) -> Self {
        use chrono::{Datelike, Timelike};

        let date = value.date();
        let time = value.time();

        DateTime {
            year: date.year() as u16,
            month: date.month() as u8,
            day: date.day() as u8,

            hour: time.hour() as u8,
            minute: time.minute() as u8,
            second: time.second() as u8,
        }
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime> for chrono::NaiveDateTime {
    fn from(value: DateTime) -> Self {
        chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(value.year as i32, value.month as u32, value.day as u32)
                .expect("Invalid date"),
            chrono::NaiveTime::from_hms_opt(value.hour as u32, value.minute as u32, value.second as u32)
                .expect("Invalid time"),
        )
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::DateTime> for DateTime {
    fn from(value: jiff::civil::DateTime) -> Self {
        let date = value.date();
        let time = value.time();

        DateTime {
            year: date.year() as u16,
            month: date.month() as u8,
            day: date.day() as u8,

            hour: time.hour() as u8,
            minute: time.minute() as u8,
            second: time.second() as u8,
        }
    }
}

#[cfg(feature = "jiff")]
impl From<DateTime> for jiff::civil::DateTime {
    fn from(value: DateTime) -> Self {
        jiff::civil::DateTime::new(
            value.year as i16,
            value.month as i8,
            value.day as i8,
            value.hour as i8,
            value.minute as i8,
            value.second as i8,
            0,
        )
        .expect("invalid datetime")
    }
}

#[cfg(feature = "time")]
impl From<time::PrimitiveDateTime> for DateTime {
    fn from(value: time::PrimitiveDateTime) -> Self {
        let date = value.date();
        let time = value.time();

        DateTime {
            year: date.year() as u16,
            month: date.month() as u8,
            day: date.day(),

            hour: time.hour(),
            minute: time.minute(),
            second: time.second(),
        }
    }
}

#[cfg(feature = "time")]
impl From<DateTime> for time::PrimitiveDateTime {
    fn from(value: DateTime) -> Self {
        time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(
                value.year as i32,
                time::Month::try_from(value.month).expect("invalid month"),
                value.day,
            )
            .expect("invalid date"),
            time::Time::from_hms(value.hour, value.minute, value.second).expect("invalid time"),
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for DateTime {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            DateTime {
                year: u16::arbitrary(g) % 10000,
                month: u8::arbitrary(g) % 12 + 1,
                day: u8::arbitrary(g) % 28 + 1,
                hour: u8::arbitrary(g) % 24,
                minute: u8::arbitrary(g) % 60,
                second: u8::arbitrary(g) % 60,
            }
        }
    }

    #[test]
    fn basic() {
        assert_eq!(
            "20250711T22:19:00".parse::<DateTime>().unwrap(),
            DateTime {
                year: 2025,
                month: 7,
                day: 11,
                hour: 22,
                minute: 19,
                second: 0
            }
        );
    }

    #[quickcheck]
    fn roundtrip(dt: DateTime) {
        assert_eq!(dt.to_string().parse::<DateTime>().unwrap(), dt);
    }
}
