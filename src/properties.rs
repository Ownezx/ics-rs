use chrono::{DateTime, Utc};

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
            .expect("Did not find the property linked with identifier");

        Property::try_from(index).unwrap()
    }

    pub fn parse_property(line: String) -> (Property, ParserResult) {
        let mut splitted_line = line.split(':');
        let property = Property::get_property_from_identifier(splitted_line.next().unwrap());

        match property {
            Property::DTStamp
            | Property::Completed
            | Property::Created
            | Property::DTStart
            | Property::LastModified
            | Property::RecurrenceID
            | Property::ExDate
            | Property::RDate => todo!(), // Date identifier

            Property::UID // We might want to add a specific validator for UID
            | Property::Description
            | Property::Location
            | Property::Summary
            | Property::Comment
            | Property::RelatedTo
            | Property::Resources
            | Property::Categories => todo!(), // String identifier

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

        (property, ParserResult::Geo(1., 1.))
    }
}

pub enum ParserResult {
    String(String),
    DateTime(DateTime<Utc>),
    Geo(f32, f32),
}
