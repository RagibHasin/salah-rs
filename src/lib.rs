// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

//! An Islamic prayer time implementation based on the [Adhan](https://github.com/batoulapps/Adhan) library by Batoul Apps.
//! While it has a lot of commnalities with the interface has
//! been changed slightly to make it more ergonomic and intuitive.
//!
//! ##### Example
//!
//! ```
//! use salah::prelude::*;
//!
//! let new_york_city = Coordinates::new(40.7128, -74.0059);
//! let date          = Local.ymd(2019, 1, 25);
//! let params        = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
//! let prayers       = PrayerTimes::calculate(date, new_york_city, params);
//! ```

#![deny(nonstandard_style, unused, future_incompatible)]

mod astronomy;
mod models;
mod schedule;

pub use crate::astronomy::{qiblah::Qiblah, unit::Coordinates};
pub use crate::models::{
    adjustments::TimeAdjustments, high_altitude_rule::HighLatitudeRule, madhab::Madhab,
    method::Method, parameters::Parameters, prayer::Prayer,
};
pub use crate::schedule::PrayerTimes;
pub use chrono::{Date, DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};

/// A convenience module appropriate for glob imports (`use salah::prelude::*;`).
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::astronomy::unit::Coordinates;
    #[doc(no_inline)]
    pub use crate::models::{
        high_altitude_rule::HighLatitudeRule, madhab::Madhab, method::Method,
        parameters::Parameters, prayer::Prayer,
    };
    #[doc(no_inline)]
    pub use crate::schedule::PrayerTimes;
    #[doc(no_inline)]
    pub use chrono::{Date, DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_prayer_times() {
        let local_date = Local.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let schedule = PrayerTimes::calculate(local_date, coordinates, params);

        assert_eq!(
            schedule
                .time(Prayer::Fajr)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "8:42 AM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Sunrise)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "10:08 AM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Dhuhr)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "5:21 PM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Asr)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "10:22 PM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Maghrib)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "12:32 AM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Isha)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "1:57 AM"
        );
    }

    #[test]
    fn calculate_qiyam_times() {
        let date = Local.ymd(2015, 7, 12);
        // let qiyam_date  = Local.ymd(2015, 7, 13);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let schedule = PrayerTimes::calculate(date, coordinates, params);

        // Today's Maghrib: 2015-07-13T00:32:00Z
        // Tomorrow's Fajr: 2015-07-13T08:43:00Z
        // Middle of Night: 2015-07-13T04:38:00Z
        // Last Third     : 2015-07-13T05:59:00Z
        assert_eq!(
            schedule
                .time(Prayer::Maghrib)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "12:32 AM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Qiyam)
                .with_timezone(&Utc)
                .format("%-l:%M %p")
                .to_string(),
            "5:59 AM"
        );
    }
}
