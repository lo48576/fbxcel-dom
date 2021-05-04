//! Creation timestamp.

use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU64;

use anyhow::anyhow;

use crate::v7400::{Error, Result};

/// Timestamp of an FBX file creation.
///
/// This would be different from the filesystem metadata.
/// This timestamp is set by SDK, and is held inside an FBX document as a data.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CreationTimestamp {
    /// Inner value.
    ///
    /// Leap second is represented in milliseconds, i.e. 23:59:60.999 is
    /// represented as `{ hour: 23, minute: 59, second: 59, millisecond: 1999 }`.
    /// In this way, unix time can be generated as if there is no leap year.
    /// For detail, see
    /// <https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveTime.html#leap-second-handling>.
    ///
    ///
    /// Bit offsets (0 is LSB, 63 is MSB):
    ///
    /// * millisecond: 0..11
    /// * second: 11..17
    /// * minute: 17..23
    /// * hour: 23..28
    /// * mday1: 28..33
    /// * month1: 33..37
    /// * year: 37..53
    inner: NonZeroU64,
}

macro_rules! bits {
    ($expr:expr, $offset:expr, $width:expr) => {
        ($expr >> $offset) & !(!0 << $width)
    };
}

impl CreationTimestamp {
    /// Returns the year.
    #[inline]
    #[must_use]
    pub fn year(self) -> u32 {
        bits!(self.inner.get(), 37, 16) as u32
    }

    /// Returns the month.
    ///
    /// The first month is 1.
    #[inline]
    #[must_use]
    pub fn month1(self) -> u32 {
        bits!(self.inner.get(), 33, 4) as u32
    }

    /// Returns the day of a month.
    ///
    /// The first day of a month is 1.
    #[inline]
    #[must_use]
    pub fn mday1(self) -> u32 {
        bits!(self.inner.get(), 28, 5) as u32
    }

    /// Returns a tuple of the year, the day and the day of a month.
    ///
    /// The first month is 1.
    /// The first day of a month is 1.
    pub fn ym1d1(self) -> (u16, u8, u8) {
        (self.year() as u16, self.month1() as u8, self.mday1() as u8)
    }

    /// Returns the hour.
    ///
    /// The first hour is 0.
    #[inline]
    #[must_use]
    pub fn hour(self) -> u32 {
        bits!(self.inner.get(), 23, 5) as u32
    }

    /// Returns the minute.
    ///
    /// The first minute is 0.
    #[inline]
    #[must_use]
    pub fn minute(self) -> u32 {
        bits!(self.inner.get(), 17, 6) as u32
    }

    /// Returns the second.
    ///
    /// The first second is 0.
    #[inline]
    #[must_use]
    pub fn second(self) -> u32 {
        bits!(self.inner.get(), 11, 6) as u32
    }

    /// Returns a tuple of the hour, the minute, and the second.
    pub fn hms(self) -> (u8, u8, u8) {
        (self.hour() as u8, self.minute() as u8, self.second() as u8)
    }

    /// Returns the millisecond.
    ///
    /// The first millisecond is 0.
    #[inline]
    #[must_use]
    pub fn millisecond(self) -> u32 {
        bits!(self.inner.get(), 0, 11) as u32
    }

    /// Returns the day of a year.
    ///
    /// The first day of a year is 0.
    #[inline]
    #[must_use]
    fn yday0(self) -> u16 {
        /// Day of a year of the first day in each month.
        const YDAY0_OF_MONTH: [u16; 12] = [
            0,   // Begining.
            31,  // 0+31.
            59,  // 31+28.
            90,  // 59+31.
            120, // 90+30.
            151, // 120+31.
            181, // 151+30.
            212, // 181+31.
            243, // 212+31.
            273, // 243+30.
            304, // 273+31.
            334, // 304+30.
        ];
        let mday0 = self.mday1() - 1;
        let month0 = self.month1() as usize - 1;
        assert!(
            month0 < 12,
            "valid month0 should be in 0..=11, but got {}",
            month0
        );
        let leap_year_offset = if is_leap_year(self.year()) { 1 } else { 0 };
        YDAY0_OF_MONTH[month0] + mday0 as u16 + leap_year_offset
    }

    /// Returns the unix time.
    ///
    /// Note that a unix time has a precision of seconds.
    /// To get nanoseconds, use [`nanosecond`][`Self::nanosecond`] method.
    ///
    /// Note that this does not consider the timezone and treat the creation
    /// timestamp as UTC time.
    /// You should adjust the result using appropriate time offset to get local time.
    pub fn seconds_since_epoch(self) -> u64 {
        /// Seconds in a day.
        const SEC_IN_DAY: u64 = 365 * 86400;

        // See
        // <https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_16>.
        let seconds_in_year = u64::from(self.second())
            + u64::from(self.minute()) * 60
            + u64::from(self.hour()) * 3600
            + u64::from(self.yday0()) * 86400;
        let year = u64::from(self.year());
        let year_seconds_offset = (year - 70) * (365 * SEC_IN_DAY) + ((year - 69) / 4) * SEC_IN_DAY
            - ((year - 1) / 100) * SEC_IN_DAY
            + ((year + 299) / 400) * SEC_IN_DAY;

        seconds_in_year + year_seconds_offset
    }

    /// Creates a timestamp from a `RawCreationTimestamp`.
    pub(super) fn from_raw(raw: RawCreationTimestamp) -> Result<Self> {
        /// Days of each month.
        const DAYS_OF_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        assert!(raw.year <= 9999, "year should be already validated");
        assert!(
            (raw.month1 >= 1) && (raw.month1 <= 12),
            "month1 should be already validated"
        );
        let days_of_the_month = if (raw.month1 == 2) && is_leap_year(u32::from(raw.year)) {
            29
        } else {
            DAYS_OF_MONTH[raw.month1 as usize - 1]
        };
        if (raw.mday1 < 1) || (raw.mday1 > days_of_the_month) {
            return Err(Error::new(anyhow!(
                "invalid day of a month: {:04}-{:02}-{:02}",
                raw.year,
                raw.month1,
                raw.mday1
            )));
        }

        assert!(raw.hour <= 23, "hour should be already validated");
        assert!(raw.minute <= 59, "minute should be already validated");
        let (second, is_leap_second) = match raw.second.cmp(&60) {
            Ordering::Greater => {
                return Err(Error::new(anyhow!(
                    "invalid time: {:02}:{:02}:{:02}",
                    raw.hour,
                    raw.minute,
                    raw.second
                )))
            }
            Ordering::Equal => (59, true),
            Ordering::Less => (raw.second, false),
        };
        if raw.millisecond >= 2000 {
            return Err(Error::new(anyhow!(
                "invalid millisecond: .{:03}",
                raw.millisecond
            )));
        }
        if is_leap_second && raw.millisecond >= 1000 {
            return Err(Error::new(anyhow!(
                "invalid leap second representation: {:02}:{:02}:{:02}.{:03}",
                raw.hour,
                raw.minute,
                raw.second,
                raw.millisecond
            )));
        }
        let millisecond = if is_leap_second {
            raw.millisecond + 1000
        } else {
            raw.millisecond
        };

        let bits = u64::from(millisecond)
            | u64::from(second) << 11
            | u64::from(raw.minute) << 17
            | u64::from(raw.hour) << 23
            | u64::from(raw.mday1) << 28
            | u64::from(raw.month1) << 33
            | u64::from(raw.year) << 37;
        Ok(Self {
            inner: NonZeroU64::new(bits).expect("should never fail: mday1 and month1 is nonzero"),
        })
    }
}

impl fmt::Debug for CreationTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print in RFC3339 `full-date` and `partial-time`.
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            self.year(),
            self.month1(),
            self.mday1(),
            self.hour(),
            self.minute(),
            self.second(),
            self.millisecond()
        )
    }
}

/// Raw timestamp of an FBX file creation.
///
/// This would be different from the filesystem metadata.
/// This timestamp is set by SDK, and is held inside an FBX document as a data.
///
/// # Notes
///
/// * It is unknown how leap seconds are handled by the official FBX SDK.
/// * This value might be invalid as a datetime.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawCreationTimestamp {
    /// Year.
    // 0..=9999.
    year: u16,
    /// Month.
    ///
    /// The first month is 1.
    // 1..=12.
    month1: u8,
    /// Day of a month.
    ///
    /// The first mday is 1.
    // 1..=31.
    mday1: u8,
    /// Hour.
    // 0..=23.
    hour: u8,
    /// Minute.
    // 0..=59.
    minute: u8,
    /// Second.
    ///
    /// Usually the value is in `0..=59`.
    /// However, it is unknown how a leap second is represented, so users should
    /// be able to handle the value `60`.
    // Assuming 0..=60. Possibly 0..=59?
    second: u8,
    ///
    /// Usually the value is in `0..=999`.
    /// However, it is unknown how a leap second is represented, so users should
    /// be able to handle the value up to `1999`.
    // Assuming 0..=1999. Possibly 0..=999?
    millisecond: u16,
}

impl RawCreationTimestamp {
    /// Creates a new value.
    #[inline]
    #[must_use]
    pub(super) fn new(
        year: u16,
        month1: u8,
        mday1: u8,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Self {
        Self {
            year,
            month1,
            mday1,
            hour,
            minute,
            second,
            millisecond,
        }
    }

    /// Returns the year.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    #[inline]
    #[must_use]
    pub fn year_raw(&self) -> u16 {
        self.year
    }

    /// Returns the month.
    ///
    /// The first month is 1.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    #[inline]
    #[must_use]
    pub fn month1_raw(&self) -> u8 {
        self.month1
    }

    /// Returns the day of a month.
    ///
    /// The first day of a month is 1.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    #[inline]
    #[must_use]
    pub fn mday1_raw(&self) -> u8 {
        self.mday1
    }

    /// Returns the hour.
    ///
    /// The first hour is 0.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    #[inline]
    #[must_use]
    pub fn hour_raw(&self) -> u8 {
        self.hour
    }

    /// Returns the minute.
    ///
    /// The first minute is 0.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    #[inline]
    #[must_use]
    pub fn minute_raw(&self) -> u8 {
        self.minute
    }

    /// Returns the second.
    ///
    /// The first second is 0.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    /// Also note that it is unknown how leap second is represented.
    /// Users should be able to handle the value up to `60`.
    #[inline]
    #[must_use]
    pub fn second_raw(&self) -> u8 {
        self.second
    }

    /// Returns the second.
    ///
    /// The first second is 0.
    ///
    /// Note that this value is the raw value from the document, and might be invalid.
    /// Also note that it is unknown how leap second is represented.
    /// Users should be able to handle the value up to `1999`.
    #[inline]
    #[must_use]
    pub fn millisecond_raw(&self) -> u16 {
        self.millisecond
    }
}

/// Returns whether the year is a leap year.
#[inline] // Used at few place.
#[must_use]
fn is_leap_year(year: u32) -> bool {
    // wrapping_{add,sub}: These addition and subtraction never overflow.
    u32::from(year % 4 != 0)
        .wrapping_sub(u32::from(year % 100 != 0))
        .wrapping_add(u32::from(year % 400 != 0))
        != 0
}
