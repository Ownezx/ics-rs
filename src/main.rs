use properties::time_properties::Created;

use chrono::{TimeZone, Utc};
mod properties;

fn main() {
    let var = Utc.ymd(2014, 7, 8).and_hms(16, 10, 11);
}
