use libbdgt::datetime::{Clock, Timestamp};

use chrono::{TimeZone, Datelike};

use crate::error::{Result, Error};
use crate::errors;


/// Type for representing years
type Year = i32;

/// Type for representing months
type Month = u32;

/// Type for representing days
type Day = u32;


/// Naive duration representation
pub(crate) enum Duration {
    /// Signed shift in years
    Year(i32),

    /// Signed shift in months
    Month(i32),
}


/// Get absolute month from relative to current date.
/// 
/// If month is in range \[1, 12\], then the function returns 
/// the value unchanged. Value 0 is translated into current
/// month, value -n is translated into the n-th previous month.
/// Positive value n greater than 12 translates into the
/// (n - 12)-th next month.
/// 
/// * `relative_month` - month relative to the current one
pub(crate) fn absolute_month(relative_month: i32) -> Month {
    if 1 <= relative_month && relative_month <= 12 {
        return relative_month as Month;
    }

    let current_month = Clock::now().month() as i32;
    match (current_month + relative_month) % 12 {
        0 => 12 as Month,
        m => m as Month
    }
}


/// Get absolute year from relative to current date.
/// 
/// * `relative_month` - year relative to the current one
pub(crate) fn absolute_year(relative_year: i32) -> Year {
    if relative_year > 0 {
        return relative_year as Year;
    }

    let current_year = Clock::now().year();
    current_year + relative_year
}


/// Creates a datetime object from  a calendar date (year, month and day).
pub(crate) fn make_date(year: Year, month: Month, day: Day) -> Result<Timestamp> {
    let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(Error::from_message_with_extra(errors::INVALID_DATE, 
            format!("year {year}, month {month}, day {day}")))?;

    let time = chrono::NaiveTime::from_hms_opt(0, 0, 0)
            .expect("Midnight is a valid time");

    Ok(chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::new(date, time)))
}


/// Shifts a datetime by the specified duration.
/// 
/// * `origin` - date to shift
/// * `shift` - duration to shift by
pub(crate) fn advance_date(origin: &Timestamp, shift: Duration) -> Result<Timestamp> {
    match shift {
        Duration::Year(diff) => {
            advance_date_year(origin, diff)
        },
        Duration::Month(diff) => {
            advance_date_month(origin, diff)
        }
    }
}


fn advance_date_year(origin: &Timestamp, shift: i32) -> Result<Timestamp> {
    make_date(origin.year() + shift, origin.month(), origin.day())
}


fn advance_date_month(origin: &Timestamp, shift: i32) -> Result<Timestamp> {
    let (mut year_shift, month_shift) = (shift / 12, shift % 12);

    let origin_month = origin.month() as i32;
    let month = origin_month + month_shift;

    //
    // Since -12 < month_shift < 12, the following holds: -11 < month < 24.
    // The following rule applies:
    // 
    // +-----------------------+-------------+------------+-------------+
    // |                 month | -10, ..., 0 | 1, ..., 12 | 13, ..., 23 |
    // +-----------------------+-------------+------------+-------------+
    // |             new month |  month + 12 |      month |  month - 12 |
    // +-----------------------+-------------+------------+-------------+
    // | additional year shift |          -1 |          0 |           1 |
    // +-----------------------+-------------+------------+-------------+
    //

    let (month, additional_year_shift) = match month {
        m if m < 1 => (m + 12, -1),
        m if m > 12 => (m - 12, 1),
        m => (m, 0)
    };

    let month = month as u32;
    year_shift += additional_year_shift;

    make_date(origin.year() + year_shift, month, origin.day())
}
