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

/// `CALSCALE` Property : This property defines the calendar scale used for the calendar information specified in the iCalendar object. The Gregorian calendar scale is assumed if this property is not specified in the iCalendar object.
pub struct CalScale {} // Need to be defined more specifically as this needs specific parsing

/// `VERSION`Property : specifies the identifier corresponding to the highest version number or the minimum and maximum range of the iCalendar specification that is required in order to interpret the iCalendar object.
pub struct Version {} // Need to be defined more specifically as this needs specific parsing for validity

/// `CATEGORIES` Property : defines the categories for a calendar component.
pub struct Categories {} // Need to be defined more specifically as this needs string lists

/// `CLASS` Property : defines the access classification for a calendar component. "PUBLIC" (default) / "PRIVATE" / "CONFIDENTIAL" / iana-token / x-name
pub struct Class {} // Need to be defined more specifically as this needs string lists

/// `RESOURCE` Property : defines the equipment or resources anticipated for an activity specified by a calendar entity.
pub struct Resource {} // Need to be defined more specifically as this needs string lists

/// `RESOURCE` Property : defines the overall status or confirmation for the calendar component.
pub struct Status {} // Need to be defined more specifically as this is specific for different types of event

/// `TRANSP` Property : defines whether an event is transparent or not to busy time searches. Valid values : OPAQUE, TRANSPARENT.
pub struct Transp {} // Need to be defined more specifically as this is specific for different types of event
