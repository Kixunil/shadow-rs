use crate::Format;
use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat, TimeZone, Utc};

pub enum BuildTime {
    Local(DateTime<Local>),
    Utc(DateTime<Utc>),
}

pub fn now_data_time() -> BuildTime {
    // Enable reproducibility for uses of `now_data_time` by respecting the
    // `SOURCE_DATE_EPOCH` env variable.
    //
    // https://reproducible-builds.org/docs/source-date-epoch/
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    match std::env::var_os("SOURCE_DATE_EPOCH") {
        None => BuildTime::Local(Local::now()),
        Some(timestamp) => {
            let epoch = timestamp
                .into_string()
                .expect("Input SOURCE_DATE_EPOCH could not be parsed")
                .parse::<i64>()
                .expect("Input SOURCE_DATE_EPOCH could not be cast to a number");
            BuildTime::Utc(Utc.timestamp(epoch, 0))
        }
    }
}

impl BuildTime {
    pub fn local_now() -> Self {
        BuildTime::Local(Local::now())
    }

    pub fn timestamp_2_utc(time_stamp: i64) -> Self {
        let dt = NaiveDateTime::from_timestamp(time_stamp, 0);
        BuildTime::Utc(DateTime::<Utc>::from_utc(dt, Utc))
    }

    pub fn to_rfc2822(&self) -> String {
        match self {
            BuildTime::Local(dt) => dt.to_rfc2822(),
            BuildTime::Utc(dt) => dt.to_rfc2822(),
        }
    }

    pub fn to_rfc3339_opts(&self, secform: SecondsFormat, use_z: bool) -> String {
        match self {
            BuildTime::Local(dt) => dt.to_rfc3339_opts(secform, use_z),
            BuildTime::Utc(dt) => dt.to_rfc3339_opts(secform, use_z),
        }
    }
}

impl Format for BuildTime {
    fn human_format(&self) -> String {
        let fmt = "%Y-%m-%d %H:%M:%S %:z";
        match self {
            BuildTime::Local(dt) => dt.format(fmt).to_string(),
            BuildTime::Utc(dt) => dt.format(fmt).to_string(),
        }
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
}
