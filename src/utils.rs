use chrono::Utc;
use nostr_sdk::nostr::Timestamp;
use rust_i18n::t;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct DiffDayHourMinuteSecond {
    pub seconds: i16,
    pub hours: i16,
    pub minutes: i16,
    pub days: i16,
    pub months: i16,
    pub years: i16,
}

impl DiffDayHourMinuteSecond {
    pub fn calc_diff_hours(t: Timestamp) -> Option<DiffDayHourMinuteSecond> {
        let now = Timestamp::now();
        let mut diff_seconds = now.as_i64() - t.as_i64();
        if diff_seconds > 0 {
            let years = diff_seconds / YEAR_SECONDS;
            diff_seconds -= years * YEAR_SECONDS;
            let months = diff_seconds / MONTH_SECONDS;
            diff_seconds -= months * MONTH_SECONDS;
            let days = diff_seconds / DAY_SECONDS;
            diff_seconds -= days * DAY_SECONDS;
            let hours = diff_seconds / HOUR_SECONDS;
            diff_seconds -= hours * HOUR_SECONDS;
            let minutes = diff_seconds / MINUTE_SECONDS;
            diff_seconds -= minutes * MINUTE_SECONDS;
            return Some(DiffDayHourMinuteSecond {
                seconds: diff_seconds as i16,
                hours: hours as i16,
                minutes: minutes as i16,
                days: days as i16,
                years: years as i16,
                months: months as i16,
            });
        }
        None
    }

    fn inner_to_string(&self) -> String {
        let v = |v: i16| if v > 1 { "s" } else { "" };
        let mut ret: Vec<_> = Vec::with_capacity(18);
        if self.years > 0 {
            ret.push(format!("{}", self.years));
            ret.push(t!("year", plural = v(self.years)));
            ret.push(",".to_owned());
        }
        if self.months > 0 {
            ret.push(format!("{}", self.months));
            ret.push(t!("month", plural = v(self.months)));
            ret.push(",".to_owned());
        }
        if self.days > 0 {
            ret.push(format!("{}", self.days));
            ret.push(t!("day", plural = v(self.days)));
            ret.push(",".to_owned());
        }
        if self.hours > 0 {
            ret.push(format!("{}", self.hours));
            ret.push(t!("hour", plural = v(self.hours)));
        }
        if self.minutes > 0 {
            ret.push(format!("{}", self.minutes));
            ret.push(t!("minute", plural = v(self.minutes)));
        }
        if self.seconds > 0 {
            ret.push(format!("{}", self.seconds));
            ret.push(t!("second", plural = v(self.seconds)));
        }
        if ret.is_empty() {
            "".to_owned()
        } else {
            format!("{}{}", ret.join(""), t!("ago"))
        }
    }
}
const YEAR_SECONDS: i64 = 365 * 86400;
const MONTH_SECONDS: i64 = 30 * 86400;
const DAY_SECONDS: i64 = 86400;
const HOUR_SECONDS: i64 = 3600;
const MINUTE_SECONDS: i64 = 60;

impl Display for DiffDayHourMinuteSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner_to_string())
    }
}

pub fn calc_diff_hours_string(t: Timestamp) -> String {
    if let Some(d) = DiffDayHourMinuteSecond::calc_diff_hours(t) {
        d.to_string()
    } else {
        "".to_owned()
    }
}

const TS_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const TS_FORMAT1: &'static str = "%H:%M:%S";
pub fn format_local_timestamp(seconds: i64) -> String {
    use chrono::{Days, Local, LocalResult, TimeZone};
    match Utc.timestamp_opt(seconds, 0) {
        LocalResult::Single(utc) | LocalResult::Ambiguous(utc, _) => {
            let now = Utc::now();
            let now_native = now.date_naive();
            let utc_native = utc.date_naive();
            // Local.from_utc_datetime(utc)
            let local_dt = Local.from_utc_datetime(&utc.naive_local());
            // fixme: transfer utc to local
            if utc_native == now_native {
                local_dt.format(TS_FORMAT1).to_string()
            } else if now_native.checked_sub_days(Days::new(1)).unwrap() == utc_native {
                format!(
                    "{}{}",
                    t!("yesterday"),
                    local_dt.format(TS_FORMAT1).to_string()
                )
            } else {
                local_dt.format(TS_FORMAT).to_string()
            }
        }
        LocalResult::None => "".to_owned(),
    }
}

pub fn front_n_chars<T: AsRef<str>>(data: T, n: usize) -> String {
    data.as_ref()
        .chars()
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| if i < n { Some(c) } else { None })
        .fold("".to_owned(), |acc, val| format!("{acc}{val}"))
}
