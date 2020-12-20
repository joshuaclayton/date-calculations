//! This crate provides helper functions for calculating shifts in Chrono's NaiveDate values for
//! various periods (week, month, quarter, year) for common shifts in direction (beginning_of_*,
//! end_of_*, previous_*, and next_*).
//!
//! The dates passed to these functions should be Gregorian dates to ensure proper calcuation.
//!
//! ```
//! use chrono::prelude::*;
//! use date_calculations::*;
//!
//! let twenty_twenty_one = NaiveDate::from_ymd_opt(2021, 1, 31).unwrap();
//!
//! assert_eq!(next_year(&twenty_twenty_one).unwrap().year(), 2022);
//! assert_eq!(next_year(&twenty_twenty_one).unwrap().month(), 1);
//! assert_eq!(next_year(&twenty_twenty_one).unwrap().day(), 1);
//!
//! assert_eq!(previous_quarter(&twenty_twenty_one).unwrap().year(), 2020);
//! assert_eq!(previous_quarter(&twenty_twenty_one).unwrap().month(), 10);
//! assert_eq!(previous_quarter(&twenty_twenty_one).unwrap().day(), 1);
//! ```

use chrono::prelude::*;

// weeks

pub fn beginning_of_week(date: &NaiveDate) -> Option<NaiveDate> {
    if date.weekday() == Weekday::Sun {
        Some(date.clone())
    } else {
        NaiveDate::from_isoywd_opt(date.iso_week().year(), date.iso_week().week(), Weekday::Sun)
            .map(|d| d - chrono::Duration::weeks(1))
    }
}

pub fn end_of_week(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_week(date).map(|d| d + chrono::Duration::days(6))
}

pub fn next_week(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_week(date).map(|d| d + chrono::Duration::weeks(1))
}

pub fn previous_week(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_week(date).map(|d| d - chrono::Duration::weeks(1))
}

pub fn beginning_of_month(date: &NaiveDate) -> Option<NaiveDate> {
    date.with_day(1)
}

pub fn end_of_month(date: &NaiveDate) -> Option<NaiveDate> {
    next_month(date).map(|d| d - chrono::Duration::days(1))
}

pub fn next_month(date: &NaiveDate) -> Option<NaiveDate> {
    if date.month() == 12 {
        next_year(date)
    } else {
        beginning_of_month(date)?.with_month(date.month() + 1)
    }
}

pub fn previous_month(date: &NaiveDate) -> Option<NaiveDate> {
    if date.month() == 1 {
        beginning_of_month(date)?
            .with_month(12)?
            .with_year(date.year() - 1)
    } else {
        beginning_of_month(date)?.with_month(date.month() - 1)
    }
}

pub fn beginning_of_quarter(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_month(date)?.with_month(quarter_month(date))
}

pub fn end_of_quarter(date: &NaiveDate) -> Option<NaiveDate> {
    next_quarter(date).map(|d| d - chrono::Duration::days(1))
}

pub fn next_quarter(date: &NaiveDate) -> Option<NaiveDate> {
    if date.month() >= 10 {
        beginning_of_year(date)?.with_year(date.year() + 1)
    } else {
        beginning_of_month(date)?.with_month(quarter_month(date) + 3)
    }
}

pub fn previous_quarter(date: &NaiveDate) -> Option<NaiveDate> {
    if date.month() < 4 {
        beginning_of_month(date)?
            .with_year(date.year() - 1)?
            .with_month(10)
    } else {
        beginning_of_month(date)?.with_month(quarter_month(date) - 3)
    }
}

fn quarter_month(date: &NaiveDate) -> u32 {
    1 + 3 * ((date.month() - 1) / 3)
}

pub fn beginning_of_year(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_month(date)?.with_month(1)
}

pub fn end_of_year(date: &NaiveDate) -> Option<NaiveDate> {
    date.with_month(12)?.with_day(31)
}

pub fn next_year(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_year(date)?.with_year(date.year() + 1)
}

pub fn previous_year(date: &NaiveDate) -> Option<NaiveDate> {
    beginning_of_year(date)?.with_year(date.year() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::clamp;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Clone, Debug)]
    struct NaiveDateWrapper(NaiveDate);

    #[quickcheck]
    fn beginning_of_week_works(d: NaiveDateWrapper) -> bool {
        let since = d.0.signed_duration_since(beginning_of_week(&d.0).unwrap());

        beginning_of_week(&d.0).unwrap().weekday() == Weekday::Sun
            && since.num_days() >= 0
            && since.num_days() < 7
    }

    #[quickcheck]
    fn end_of_week_works(d: NaiveDateWrapper) -> bool {
        end_of_week(&d.0).unwrap().weekday() == Weekday::Sat
    }

    #[quickcheck]
    fn next_week_works(d: NaiveDateWrapper) -> bool {
        let since = next_week(&d.0).unwrap().signed_duration_since(d.0);
        next_week(&d.0).unwrap().weekday() == Weekday::Sun
            && since.num_days() > 0
            && since.num_days() <= 7
    }

    #[quickcheck]
    fn previous_week_works(d: NaiveDateWrapper) -> bool {
        let since = previous_week(&d.0).unwrap().signed_duration_since(d.0);
        previous_week(&d.0).unwrap().weekday() == Weekday::Sun
            && since.num_days() <= -7
            && since.num_days() > -14
    }

    #[quickcheck]
    fn beginning_of_month_works(d: NaiveDateWrapper) -> bool {
        beginning_of_month(&d.0).unwrap().day() == 1
            && beginning_of_month(&d.0).unwrap().month() == d.0.month()
            && beginning_of_month(&d.0).unwrap().year() == d.0.year()
    }

    #[quickcheck]
    fn end_of_month_works(d: NaiveDateWrapper) -> bool {
        end_of_month(&d.0).unwrap().month() == d.0.month()
            && end_of_month(&d.0).unwrap().year() == d.0.year()
            && (end_of_month(&d.0).unwrap() + chrono::Duration::days(1))
                == next_month(&d.0).unwrap()
    }

    #[quickcheck]
    fn beginning_of_year_works(d: NaiveDateWrapper) -> bool {
        beginning_of_year(&d.0).unwrap().month() == 1
            && beginning_of_year(&d.0).unwrap().day() == 1
            && beginning_of_year(&d.0).unwrap().year() == d.0.year()
    }

    #[quickcheck]
    fn end_of_year_works(d: NaiveDateWrapper) -> bool {
        end_of_year(&d.0).unwrap().month() == 12
            && end_of_year(&d.0).unwrap().day() == 31
            && end_of_year(&d.0).unwrap().year() == d.0.year()
    }

    #[quickcheck]
    fn next_year_works(d: NaiveDateWrapper) -> bool {
        next_year(&d.0).unwrap().month() == 1
            && next_year(&d.0).unwrap().day() == 1
            && next_year(&d.0).unwrap().year() == d.0.year() + 1
    }

    #[quickcheck]
    fn previous_year_works(d: NaiveDateWrapper) -> bool {
        previous_year(&d.0).unwrap().month() == 1
            && previous_year(&d.0).unwrap().day() == 1
            && previous_year(&d.0).unwrap().year() == d.0.year() - 1
    }

    #[quickcheck]
    fn beginning_of_quarter_works(d: NaiveDateWrapper) -> bool {
        [1, 4, 7, 10].contains(&beginning_of_quarter(&d.0).unwrap().month())
            && beginning_of_quarter(&d.0).unwrap().day() == 1
            && beginning_of_quarter(&d.0).unwrap().year() == d.0.year()
    }

    #[quickcheck]
    fn end_of_quarter_works(d: NaiveDateWrapper) -> bool {
        [3, 6, 9, 12].contains(&end_of_quarter(&d.0).unwrap().month())
            && end_of_quarter(&d.0)
                .map(|x| x + chrono::Duration::days(1))
                .unwrap()
                == next_quarter(&d.0).unwrap()
            && end_of_quarter(&d.0).unwrap().year() == d.0.year()
    }

    #[quickcheck]
    fn next_quarter_works(d: NaiveDateWrapper) -> bool {
        let current_month = d.0.month();
        let year = if current_month >= 10 {
            d.0.year() + 1
        } else {
            d.0.year()
        };

        [1, 4, 7, 10].contains(&next_quarter(&d.0).unwrap().month())
            && next_quarter(&d.0).unwrap().day() == 1
            && next_quarter(&d.0).unwrap().year() == year
    }

    #[quickcheck]
    fn previous_quarter_works(d: NaiveDateWrapper) -> bool {
        let current_month = d.0.month();
        let year = if current_month <= 3 {
            d.0.year() - 1
        } else {
            d.0.year()
        };

        [1, 4, 7, 10].contains(&previous_quarter(&d.0).unwrap().month())
            && previous_quarter(&d.0).unwrap().day() == 1
            && previous_quarter(&d.0).unwrap().year() == year
    }

    impl Arbitrary for NaiveDateWrapper {
        fn arbitrary<G: Gen>(g: &mut G) -> NaiveDateWrapper {
            let year = clamp(i32::arbitrary(g), 1584, 2800);
            let month = 1 + u32::arbitrary(g) % 12;
            let day = 1 + u32::arbitrary(g) % 31;

            let first_date = NaiveDate::from_ymd_opt(year, month, day);
            if day > 27 {
                let result = vec![
                    first_date,
                    NaiveDate::from_ymd_opt(year, month, day - 1),
                    NaiveDate::from_ymd_opt(year, month, day - 2),
                ]
                .into_iter()
                .filter_map(|v| v)
                .nth(0)
                .unwrap();

                NaiveDateWrapper(result)
            } else {
                NaiveDateWrapper(first_date.unwrap())
            }
        }
    }
}
