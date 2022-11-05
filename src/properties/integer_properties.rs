// Creation and conversion from builder types to Property
macro_rules! integer_property {
    ($type:ident, $name:expr, $description:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property : "]
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type {
            value: isize,
        }

        impl $type {
            pub fn new(value: isize) -> $type {
                $type { value }
            }

            pub fn to_string(&self) -> isize {
                self.value
            }
            pub fn write(&self) -> String {
                format!("{}:{}", $name, self.value)
            }
        }
    };
}

macro_rules! integer_property_with_validation_range {
    ($type:ident, $name:expr, $description:expr, $min:literal, $max:literal) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property : "]
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type {
            value: isize,
        }

        impl $type {
            pub fn new(value: isize) -> $type {
                if value > $max {
                    panic!("Expected a max value of {}, got {}.", $max, value)
                }
                if value < $min {
                    panic!("Expected a min value of {}, got {}.", $min, value)
                }
                $type { value }
            }

            pub fn to_string(&self) -> &isize {
                &self.value
            }
            pub fn write(&self) -> String {
                format!("{}:{}", $name, self.value)
            }
        }
    };
}

integer_property!(
    Sequence,
    "SEQUENCE",
    "Must be incremented when one of the following is modified : 
    DTSTART,DTEND, DUE, RDATE, RRULE, EXDATE, EXRULE, STATUS"
);

integer_property_with_validation_range!(
    PercentComplete,
    "PERCENT-COMPLETE",
    "used by an assignee or delegatee of a to-do to
    convey the percent completion of a to-do to the Organizer.",
    0,
    100
);

integer_property_with_validation_range!(
    Priority,
    "PRIORITY",
    "defines the relative priority for a calendar component. 0 is high, 9 is low.",
    0,
    9
);
