use chrono::{DateTime, FixedOffset, format::Fixed, Duration};

use self::uri::Uri;

pub mod cal_adress;
pub mod class;
pub mod status;
pub mod uri;

const PROPERTY_IDENTIFIER: &[&str] = &[
    // Time properties
    "DTSTAMP",
    "COMPLETED",
    "CREATED",
    "DTSTART",
    "LAST-MODIFIED",
    "RECURRENCE-ID",
    "EXDATE",
    "RDATE",
    "DUE",
    // String
    "UID",
    "DESCRIPTION",
    "LOCATION",
    "SUMMARY",
    "COMMENT",
    "RELATED-TO",
    "RESOURCES",
    "CATEGORIES",
    // Cal address
    "ORGANIZER",
    "ATTENDEE",
    "CONTACT",
    // Integer
    "PERCENT-COMPLETE",
    "PRIORITY",
    "SEQUENCE",
    // Status
    "STATUS",
    // URI
    "URL",
    "ATTACH",
    // Others
    "GEO",
    "CLASS",
];

// This was yoinked here : https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<usize> for $name {
            type Error = ();

            fn try_from(v: usize) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as usize => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
pub enum Property {
    // Time properties
    DTStamp,
    Completed,
    Created,
    DTStart,
    LastModified,
    RecurrenceID,
    ExDate,
    RDate,
    Due,

    // Duration property
    Duration,

    // String properties
    UID,
    Description,
    Location,
    Summary,
    Comment,
    RelatedTo,
    Resources,
    Categories,

    // Cal adress properties
    Organizer,
    Attendee,
    Contact,

    // Integer properties
    PercentComplete,
    Priority,
    Sequence,

    // Status,
    Status,

    // URI properties
    URL,
    Attach,

    // Others
    Geo,
    Class,
}
}

impl Property {
    pub fn get_property_from_identifier(identifier: &str) -> Property {
        let index = PROPERTY_IDENTIFIER
            .iter()
            .position(|&r| r == identifier)
            .unwrap_or_else(||panic!("Did not find the property linked with identifier {}.", identifier));

        Property::try_from(index).unwrap()
    }

    pub fn get_identier<'a>(self)-> &'a str{
        PROPERTY_IDENTIFIER[self as usize]
    }

    pub fn parse_property(line: String) -> (Property, ParserResult) {
        // This line has the parameters on one side and the values on the other.
        let mut splitted_line = line.split_once(':').unwrap();
        let mut parameters = splitted_line.0.split(';');
        
        let property = Property::get_property_from_identifier(parameters.next().unwrap());


        let result :ParserResult;
        match property {
            // Time identifier
            Property::DTStamp
            | Property::Completed
            | Property::Created
            | Property::DTStart
            | Property::LastModified
            | Property::RecurrenceID
            | Property::ExDate
            | Property::RDate
            | Property::Due => {
                // This is needed as parse_from_str wants timezone information.
                let mut temp_string = splitted_line.1.to_string();
                temp_string.push_str("+0000");
                let date_time = DateTime::parse_from_str(temp_string.as_str(), "%Y%m%dT%H%M%SZ%z").unwrap();
                result = ParserResult::DateTime(date_time);
            } 
            // Duration property
            Property::Duration => todo!(),
            // String identifier
            Property::UID // We might want to add a specific validator for UID
            | Property::Description
            | Property::Location
            | Property::Summary
            | Property::Comment
            | Property::RelatedTo
            | Property::Resources
            | Property::Categories => result = ParserResult::String(String::from(splitted_line.1)), 

            Property::Organizer
            | Property::Attendee
            | Property::Contact=>todo!(),

            Property::PercentComplete
            | Property::Priority
            | Property::Sequence
            | Property::Status=>todo!(),
            
            Property::URL
            | Property::Attach => todo!(),

            Property::Geo => todo!(),

            Property::Class => todo!(),
        }

        (property, result)
    }
}

pub enum ParserResult {
    String(String),
    DateTime(DateTime<FixedOffset>),
    Duration(Duration),
    Geo(f32, f32),
}
impl From<ParserResult> for DateTime<FixedOffset>{
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::DateTime(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for String{
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::String(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for Duration{
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Duration(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}