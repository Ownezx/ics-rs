use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use chrono::Date;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

fn main() {
    use chrono::{FixedOffset, TimeZone};

    let var: DateTime<FixedOffset> =
        DateTime::from_utc(Utc::now().naive_utc(), FixedOffset::east(0));

    let datetime = FixedOffset::east(0).ymd(2016, 11, 8).and_hms(0, 0, 0);
    println!("{}", &var.format("%Y%m%dT%H%M%SZ%z"));

    let time = DateTime::parse_from_str("20221110T162749Z+0000", "%Y%m%dT%H%M%SZ%z").unwrap();
    println!("{}", &time.format("%Y%m%dT%H%M%SZ"));

    let string = "P15DT5H0M20S";
}
