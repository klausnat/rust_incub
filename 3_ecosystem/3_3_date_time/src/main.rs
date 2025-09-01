use chrono::{Datelike, NaiveDate};
use std::time::Duration;
fn main() {}

const NOW: &str = "2025-09-01";

struct User {
    birthdate: NaiveDate,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        let birthdate = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap_or_else(|| panic!("Invalid birthdate: {}-{}-{}", year, month, day));
        User { birthdate }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let now_date = NaiveDate::parse_from_str(NOW, "%Y-%m-%d").expect("Invalid NOW date format");

        if self.birthdate > now_date {
            return 0;
        }

        let mut age = (now_date.year() - self.birthdate.year()) as u16;

        // если дня рождения в этом году еще не было (корректировка для "не полных лет")
        let current_month_day = (now_date.month(), now_date.day());
        let birth_month_day = (self.birthdate.month(), self.birthdate.day());

        if current_month_day < birth_month_day {age -= 1; } 

        age
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 35),   // 2025-09-01 - 1990-06-04 = 35 years
            ((1990, 7, 4), 35),   // 2025-09-01 - 1990-07-04 = 35 years
            ((1990, 9, 1), 35),   // Exactly on birthday
            ((1990, 9, 2), 34),   // Birthday hasn't occurred yet
            ((1990, 10, 4), 34),  // Birthday later this year
            ((0, 1, 1), 2025),    // 2025-09-01 - 0000-01-01 = 2025 years
            ((1970, 1, 1), 55),   // 2025-09-01 - 1970-01-01 = 55 years
            ((2019, 6, 25), 6),   // 2025-09-01 - 2019-06-25 = 6 years
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected, "Failed for date: {}-{}-{}", y, m, d);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2042, 6, 25), 0),
            ((2096, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn leap_year_cases() {
        // Born on February 29th
        let user = User::with_birthdate(2000, 2, 29);
        assert_eq!(user.age(), 25); // 2025-09-01 - 2000-02-29 = 25 years
        
        // Test non-leap year handling
        let user = User::with_birthdate(2001, 2, 28); // Not a leap year
        assert_eq!(user.age(), 24);
    }

    #[test]
    fn is_adult_cases() {
        // Adult cases
        assert!(User::with_birthdate(2000, 1, 1).is_adult()); // 25 years
        assert!(User::with_birthdate(2007, 8, 31).is_adult()); // Exactly 18 today
        assert!(User::with_birthdate(2007, 1, 1).is_adult()); // Over 18
        
        // Not adult cases
        assert!(!User::with_birthdate(2007, 9, 2).is_adult()); // Turns 18 tomorrow
        assert!(!User::with_birthdate(2010, 1, 1).is_adult()); // 15 years
        assert!(!User::with_birthdate(2025, 1, 1).is_adult()); // 0 years
        assert!(!User::with_birthdate(2042, 1, 1).is_adult()); // Future birthdate
    }

    #[test]
    fn exactly_18_today() {
        // Born on September 1, 2007 - exactly 18 years old today
        let user = User::with_birthdate(2007, 9, 1);
        assert_eq!(user.age(), 18);
        assert!(user.is_adult());
    }

    #[test]
    fn turns_18_tomorrow() {
        // Born on September 2, 2007 - turns 18 tomorrow
        let user = User::with_birthdate(2007, 9, 2);
        assert_eq!(user.age(), 17);
        assert!(!user.is_adult());
    }

    #[test]
    fn was_18_yesterday() {
        // Born on August 31, 2007 - was 18 yesterday
        let user = User::with_birthdate(2007, 8, 31);
        assert_eq!(user.age(), 18);
        assert!(user.is_adult());
    }
}

