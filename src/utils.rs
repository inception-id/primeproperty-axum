use chrono::{Datelike, NaiveDateTime, Utc};

pub fn get_start_of_month() -> NaiveDateTime {
    let now = Utc::now().naive_utc();
    let naive_date = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .expect("Couldn't get start of month");
    naive_date
        .and_hms_opt(0, 0, 0)
        .expect("Couldn't get start of month")
}
