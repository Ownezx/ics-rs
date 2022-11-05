use crate::untested_property;
use chrono::{DateTime, Utc};

// Creation and conversion from builder types to Property
macro_rules! date_time_property {
    ($type:ident, $name:expr, $description:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property : "]
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type {
            date_time: DateTime<Utc>,
        }

        impl $type {
            pub fn new(date_time: DateTime<Utc>) -> $type {
                $type { date_time }
            }

            pub fn to_string(&self) -> String {
                self.date_time.format("%Y%m%dT%H%M%SZ").to_string()
            }
            pub fn write(&self) -> String {
                format!(
                    "{}:{}",
                    $name,
                    self.date_time.format("%Y%m%dT%H%M%SZ").to_string()
                )
            }
        }
    };
}

// Creation and conversion from builder types to Property
macro_rules! date_time_property_with_list {
    ($type:ident, $name:expr, $description:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property : "]
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type {
            date_time: DateTime<Utc>,
        }

        impl $type {
            pub fn new(date_time: DateTime<Utc>) -> $type {
                $type { date_time }
            }

            pub fn to_string(&self) -> String {
                self.date_time.format("%Y%m%dT%H%M%SZ").to_string()
            }
            pub fn write(&self) -> String {
                format!(
                    "{}:{}",
                    $name,
                    self.date_time.format("%Y%m%dT%H%M%SZ").to_string()
                )
            }
        }
    };
}

date_time_property!(
    DtEnd,
    "DTEND",
    "specifies the date and time that a calendar component ends."
);
date_time_property!(
    Due,
    "DUE",
    "defines the date and time that a to-do is expected to be completed"
);
date_time_property!(
    DtStart,
    "DTSTART",
    "specifies when the calendar component begins."
);
date_time_property!(
    Completed,
    "COMPLETED",
    "defines the date and time that a to-do was actually completed."
);
date_time_property!(
    DtStamp,
    "DTSTAMP",
    "indicates the date/time that the instance of the iCalendar object was created."
);
date_time_property!(
    LastModified,
    "LAST-MODIFIED",
    "specifies the date and time that the information associated with the calendar 
    component was last revised in the calendar store. Note: This is analogous to the modification 
    date and time for a file in the file system."
);

date_time_property!(
    Created,
    "CREATED",
    "specifies the date and time that the calendar information was created by the 
    calendar user agent in the calendar store. Note: This is analogous to the creation date and 
    time for a file in the file system."
);

date_time_property_with_list!(
    ExDate,
    "EXDATE",
    "defines the list of date/time exceptions for a recurring calendar component."
);

untested_property!(
    RDate,
    "RDATE",
    "defines the list of date/times for a recurrence set."
); // This is much more complex because it can also have a duration. We might have to do a separate entity for it.
