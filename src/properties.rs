pub mod cal_address_properties;
pub mod duration_properties;
pub mod geo;
pub mod integer_properties;
pub mod recurrence_id;
pub mod string_properties;
pub mod time_properties;
pub mod uri_properties;
pub mod utc_offset_properties;
pub mod vcalendar;

/// Untested properties are treated as string properties
#[macro_export]
macro_rules! untested_property {
    ($type:ident, $name:expr, $description:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Untested property : "]
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type {
            value: String,
        }

        impl $type {
            pub fn new(value: String) -> $type {
                eprintln!("Warning, property {} is currently untested and does not validate data. Use at your own discretion", $name);
                $type { value }
            }

            pub fn to_string(&self) -> &String {
                &self.value
            }
            pub fn write(&self) -> String {
                format!("{}:{}", $name, self.value)
            }
        }
    };
}

untested_property!(
    UID,
    "UID",
    "defines the persistent, globally unique identifier for the calendar component."
);

untested_property!(
    ExRule,
    "EXRULE",
    "defines a rule or repeating pattern for an exception to a recurrence set."
);

untested_property!(
    Repeat,
    "REPEAT",
    "defines the number of time the alarm should be repeated, after the initial trigger. (It contains an integer and duration)"
);

untested_property!(Trigger, "TRIGGER", "specifies when an alarm will trigger.");
