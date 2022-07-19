use crate::Format;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::UtcOffset;
use time::{format_description, OffsetDateTime};
use tz::TimeZone;

pub enum DateTime {
    Local(OffsetDateTime),
    Utc(OffsetDateTime),
}

pub fn now_data_time() -> DateTime {
    // Enable reproducibility for uses of `now_data_time` by respecting the
    // `SOURCE_DATE_EPOCH` env variable.
    //
    // https://reproducible-builds.org/docs/source-date-epoch/
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    match std::env::var_os("SOURCE_DATE_EPOCH") {
        None => DateTime::now(),
        Some(timestamp) => {
            let epoch = timestamp
                .into_string()
                .expect("Input SOURCE_DATE_EPOCH could not be parsed")
                .parse::<i64>()
                .expect("Input SOURCE_DATE_EPOCH could not be cast to a number");
            // BuildTime::Utc(Utc.timestamp(epoch, 0))
            DateTime::Utc(OffsetDateTime::from_unix_timestamp(epoch).unwrap())
        }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl DateTime {
    pub fn now() -> Self {
        Self::local_now().unwrap_or_else(|_| DateTime::Utc(OffsetDateTime::now_utc()))
    }

    pub fn offset_datetime() -> OffsetDateTime {
        let date_time = Self::now();
        match date_time {
            DateTime::Local(time) | DateTime::Utc(time) => time,
        }
    }

    pub fn local_now() -> Result<Self, Box<dyn Error>> {
        let time_zone_local = TimeZone::local()?; // expensive call, should be cached

        let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let local_time_type =
            time_zone_local.find_local_time_type(duration_since_epoch.as_secs().try_into()?)?;
        let time_zone_offset = UtcOffset::from_whole_seconds(local_time_type.ut_offset())?;
        let local_date_time =
            (OffsetDateTime::UNIX_EPOCH + duration_since_epoch).to_offset(time_zone_offset);
        Ok(DateTime::Local(local_date_time))
    }

    pub fn timestamp_2_utc(time_stamp: i64) -> Self {
        let time = OffsetDateTime::from_unix_timestamp(time_stamp).unwrap();
        DateTime::Utc(time)
    }

    pub fn to_rfc2822(&self) -> String {
        match self {
            DateTime::Local(dt) | DateTime::Utc(dt) => dt.format(&Rfc2822).unwrap(),
        }
    }

    pub fn to_rfc3339(&self) -> String {
        match self {
            DateTime::Local(dt) | DateTime::Utc(dt) => dt.format(&Rfc3339).unwrap(),
        }
    }
}

impl Format for DateTime {
    fn human_format(&self) -> String {
        match self {
            DateTime::Local(dt) | DateTime::Utc(dt) => dt.human_format(),
        }
    }
}

impl Format for OffsetDateTime {
    fn human_format(&self) -> String {
        let fmt = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]",
        )
        .unwrap();
        self.format(&fmt).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_date_epoch() {
        std::env::set_var("SOURCE_DATE_EPOCH", "1628080443");
        let time = now_data_time();
        assert_eq!(time.human_format(), "2021-08-04 12:34:03 +00:00");
    }

    #[test]
    fn test_local_now_human_format() {
        let time = DateTime::local_now().unwrap().human_format();
        #[cfg(unix)]
        assert!(!std::fs::read("/etc/localtime").unwrap().is_empty());

        println!("{}", time); // 2022-07-14 00:40:05 +08:00
        assert_eq!(time.len(), 26);
    }

    #[test]
    fn test_timestamp_2_utc() {
        let time = DateTime::timestamp_2_utc(1628080443);
        assert_eq!(time.to_rfc2822(), "Wed, 04 Aug 2021 12:34:03 +0000");
        assert_eq!(time.to_rfc3339(), "2021-08-04T12:34:03Z");
        assert_eq!(time.human_format(), "2021-08-04 12:34:03 +00:00");
    }
}