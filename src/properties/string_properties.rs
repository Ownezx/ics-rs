use crate::untested_property;

// Creation and conversion from builder types to Property
macro_rules! string_property {
    ($type:ident, $name:expr, $description:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property : "]
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type {
            value: String,
        }

        impl $type {
            pub fn new(value: String) -> $type {
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

string_property!(
    ProdID,
    "PRODID",
    "specifies the identifier for the product that created the iCalendar object."
);

string_property!(
    Method,
    "METHOD",
    "defines the iCalendar object method associated with the calendar object."
);

string_property!(
    Comment,
    "COMMENT",
    "specifies non-processing information intended to provide a comment to the calendar user."
);

string_property!(
    Description,
    "DESCRIPTION",
    "provides a more complete description of the calendar component, than that provided by the 'SUMMARY' property."
);

string_property!(
    Location,
    "LOCATION",
    "defines the intended venue for the activity defined by a calendar component."
);

string_property!(
    Summary,
    "SUMMARY",
    "defines a short summary or subject for the calendar component."
);

string_property!(
    Tzid,
    "TZID",
    "specifies the text value that uniquely identifies the 'VTIMEZONE' calendar component."
);

string_property!(
    Tzname,
    "TZID",
    "specifies the customary designation for a time zone description."
);

string_property!(
    Tzurl,
    "TZURL",
    "The TZURL provides a means for a VTIMEZONE component to point to a network location that can be used to retrieve an up-to- date version of itself."
); // Note should be an URI property

string_property!(
    RelateedTo,
    "RELATED-TO",
    "used to represent a relationship or reference between one calendar component and another."
);

untested_property!(
    CalScale,
    "CALSCALE",
    "defines the calendar scale used for the calendar information specified in the iCalendar object. The Gregorian calendar scale is assumed if this property is not specified in the iCalendar object."
);

untested_property!(
    Action,
    "ACTION",
    "defines the action to be invoked when an alarm is triggered."
);

untested_property!(
    Version,
    "VERSION",
    "specifies the identifier corresponding to the highest version number or the minimum and maximum range of the iCalendar specification that is required in order to interpret the iCalendar object."
);

untested_property!(
    Categories,
    "VERSION",
    "defines the categories for a calendar component."
);

untested_property!(
    Class,
    "CLASS",
    "defines the access classification for a calendar component. 'PUBLIC' (default) / 'PRIVATE' / 'CONFIDENTIAL' / iana-token / x-name"
);

untested_property!(
    Resource,
    "RESOURCE",
    "defines the equipment or resources anticipated for an activity specified by a calendar entity."
);

untested_property!(
    Status,
    "RESOURCE",
    "defines the overall status or confirmation for the calendar component."
);

untested_property!(
    Transp,
    "TRANSP",
    "defines whether an event is transparent or not to busy time searches. Valid values : OPAQUE, TRANSPARENT."
);
