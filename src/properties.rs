use std::str::FromStr;

#[cfg(test)]
use chrono::TimeZone;
use chrono::{DateTime, Duration, FixedOffset};

use crate::ics_error::ICSError;

use self::{class::Class, status::Status};

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
    // Duration
    "DURATION",
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
#[derive(Debug,PartialEq, Eq)]
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
    pub fn get_property_from_identifier(identifier: &str) -> Option<Property> {
        let index = PROPERTY_IDENTIFIER.iter().position(|&r| r == identifier);

        index.map(|index| Property::try_from(index).unwrap())
    }

    pub fn get_identier<'a>(self) -> &'a str {
        PROPERTY_IDENTIFIER[self as usize]
    }

    pub fn parse_property(line: String) -> Result<(Property, ParserResult), ICSError> {
        // This line has the parameters on one side and the values on the other.
        let splitted_line = match line.split_once(':') {
            Some(l) => l,
            None => return Err(ICSError::UnableToParseProperty),
        };
        let mut parameters = splitted_line.0.split(';');

        let var = parameters.next().unwrap();
        // println!("{}",var);
        let property = Property::get_property_from_identifier(var);

        if property.is_none() {
            return Err(ICSError::UknownProperty);
        }

        let property = property.unwrap();

        let result: ParserResult = match property {
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

                // Deal with all the parameters possible for time values
                let mut parameter = parameters.next();
                while parameter.is_some() {
                    // Split the parameter string
                    let (param_name, param_value) = match parameter.unwrap().split_once('=') {
                        Some(val) => (val.0, val.1),
                        None => return Err(ICSError::PropertyConditionNotRespected),
                    };

                    // Match the parameter with different possibilities
                    match param_name {
                        "VALUE" => {
                            match param_value {
                                // If it is a date, lets add some 0 time to parse it properly
                                "DATE" => temp_string.push_str("T000000Z"),
                                "DATE-TIME" => {}
                                _ => return Err(ICSError::PropertyConditionNotRespected),
                            }
                        }
                        _ => return Err(ICSError::PropertyConditionNotRespected),
                    }

                    parameter = parameters.next();
                }

                temp_string.push_str("+0000");
                let date_time =
                    match DateTime::parse_from_str(temp_string.as_str(), "%Y%m%dT%H%M%SZ%z") {
                        Ok(value) => value,
                        Err(_) => {
                            match DateTime::parse_from_str(temp_string.as_str(), "%Y%m%dT%H%MZ%z") {
                                Ok(value) => value,
                                Err(_) => return Err(ICSError::PropertyConditionNotRespected),
                            }
                        }
                    };
                ParserResult::DateTime(date_time)
            }
            // Duration property
            Property::Duration => todo!(),
            // String identifier
            // We might want to add a specific validator for UID
            Property::UID
            | Property::Description
            | Property::Location
            | Property::Summary
            | Property::Comment
            | Property::RelatedTo
            | Property::Resources => ParserResult::String(String::from(splitted_line.1)),

            Property::Categories => {
                let mut vec: Vec<String> = Vec::new();
                let mut categories = splitted_line.1.split(',');
                let mut category = categories.next();
                while category.is_some() {
                    vec.push(category.unwrap().to_string());
                    category = categories.next();
                }
                ParserResult::Strings(vec)
            }

            Property::Organizer | Property::Attendee | Property::Contact => todo!(),

            Property::PercentComplete | Property::Priority | Property::Sequence => {
                match splitted_line.1.to_string().parse() {
                    Ok(integer) => ParserResult::Integer(integer),
                    Err(_) => return Err(ICSError::UnableToParseProperty),
                }
            }

            Property::Status => ParserResult::Status(Status::from_str(splitted_line.1)?),

            Property::URL | Property::Attach => todo!(),

            Property::Geo => {
                // Get the two floats
                let (lat, long) = match splitted_line.1.split_once(';') {
                    Some(values) => values,
                    None => return Err(ICSError::UnableToParseProperty),
                };
                let float_lat: f32 = match lat.to_string().parse() {
                    Ok(val) => val,
                    Err(_) => return Err(ICSError::UnableToParseProperty),
                };
                let float_long: f32 = match long.to_string().parse() {
                    Ok(val) => val,
                    Err(_) => return Err(ICSError::UnableToParseProperty),
                };

                if !(-90. ..=90.).contains(&float_lat) {
                    return Err(ICSError::PropertyConditionNotRespected);
                }

                if !(-180. ..=180.).contains(&float_long) {
                    return Err(ICSError::PropertyConditionNotRespected);
                }

                ParserResult::Geo(float_lat, float_long)
            }

            Property::Class => ParserResult::Class(Class::from_str(splitted_line.1)?),
        };

        Ok((property, result))
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserResult {
    String(String),
    Strings(Vec<String>),
    DateTime(DateTime<FixedOffset>),
    Duration(Duration),
    Integer(usize),
    Status(Status),
    Class(Class),
    Geo(f32, f32),
}

impl From<ParserResult> for DateTime<FixedOffset> {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::DateTime(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for String {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::String(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for Duration {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Duration(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for usize {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Integer(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for Status {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Status(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for Class {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Class(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for (f32, f32) {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Geo(lat, long) => (lat, long),
            _ => panic!("Not casting the right result"),
        }
    }
}

impl From<ParserResult> for Vec<String> {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Strings(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

#[test]
fn all_properties_properly_recognised() {
    // Date/Datetime
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 3, 13)
        .unwrap()
        .and_hms_opt(12, 34, 32)
        .unwrap();

    let (property, value) =
        Property::parse_property("DTSTAMP:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::DTStamp);

    let (property, value) =
        Property::parse_property("COMPLETED:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::Completed);

    let (property, value) =
        Property::parse_property("CREATED:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::Created);

    let (property, value) =
        Property::parse_property("DTSTART:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::DTStart);

    let (property, value) =
        Property::parse_property("LAST-MODIFIED:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::LastModified);

    let (property, value) =
        Property::parse_property("RECURRENCE-ID:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::RecurrenceID);

    let (property, value) =
        Property::parse_property("EXDATE:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::ExDate);

    let (property, value) = Property::parse_property("RDATE:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::RDate);

    let (property, value) = Property::parse_property("DUE:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
    assert_eq!(property, Property::Due);

    // String properties
    let (property, value) =
        Property::parse_property("UID:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::UID);

    let (property, value) =
        Property::parse_property("DESCRIPTION:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Description);

    let (property, value) =
        Property::parse_property("LOCATION:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Location);

    let (property, value) =
        Property::parse_property("SUMMARY:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Summary);

    let (property, value) =
        Property::parse_property("COMMENT:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Comment);

    let (property, value) =
        Property::parse_property("RELATED-TO:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::RelatedTo);

    let (property, value) =
        Property::parse_property("RESOURCES:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Resources);

    let (property, value) =
        Property::parse_property("CATEGORIES:This is a description".to_string()).unwrap();
    assert_eq!(<Vec<String>>::from(value), vec!["This is a description"]);
    assert_eq!(property, Property::Categories);

    // Integer properties
    let (property, value) = Property::parse_property("PERCENT-COMPLETE:1".to_string()).unwrap();
    assert_eq!(usize::from(value), 1);
    assert_eq!(property, Property::PercentComplete);

    let (property, value) = Property::parse_property("PRIORITY:1".to_string()).unwrap();
    assert_eq!(usize::from(value), 1);
    assert_eq!(property, Property::Priority);

    let (property, value) = Property::parse_property("SEQUENCE:1".to_string()).unwrap();
    assert_eq!(usize::from(value), 1);
    assert_eq!(property, Property::Sequence);

    // Status
    let (property, value) = Property::parse_property("STATUS:COMPLETED".to_string()).unwrap();
    assert_eq!(Status::from(value), Status::Completed);
    assert_eq!(property, Property::Status);

    // Class
    let (property, value) = Property::parse_property("CLASS:PUBLIC".to_string()).unwrap();
    assert_eq!(Class::from(value), Class::PUBLIC);
    assert_eq!(property, Property::Class);

    // Geo
    let (property, value) =
        Property::parse_property("GEO:37.386013;-122.082932".to_string()).unwrap();
    assert_eq!(<(f32, f32)>::from(value), (37.386013, -122.082_93));
    assert_eq!(property, Property::Geo);
}

#[test]
fn string_parsing_cases() {
    // String with another ':' in the parameter
    let (property, value) =
        Property::parse_property("UID:This is a description: here".to_string()).unwrap();
    assert_eq!(
        String::from(value),
        "This is a description: here".to_string()
    );
    assert_eq!(property, Property::UID);

    // Unknown property
    let result = Property::parse_property("SDQ:content".to_string());
    assert_eq!(result, Err(ICSError::UknownProperty));
}

#[test]
fn cal_address_parsing_cases() {
    let (property, value) =
        Property::parse_property("ORGANIZER:MAILTO:jane_doe@host.com".to_string()).unwrap();
    let (property, value) =
        Property::parse_property("ORGANIZER;CN=John Smith:MAILTO:jsmith@host1.com".to_string())
            .unwrap();
    let (property, value) = Property::parse_property(
        "ORGANIZER;CN=JohnSmith;DIR=\"ldap://host.com:6666/o=3DDC%20Associ
    ates,c=3DUS??(cn=3DJohn%20Smith)\":MAILTO:jsmith@host1.com"
            .to_string(),
    )
    .unwrap();
    let (property, value) = Property::parse_property(
        "ORGANIZER;SENT-BY=\"MAILTO:jane_doe@host.com\":MAILTO:jsmith@host1.com".to_string(),
    )
    .unwrap();
    let (property, value) = Property::parse_property(
        "CONTACT:Jim Dolittle\\, ABC Industries\\, +1-919-555-1234".to_string(),
    )
    .unwrap();
    let (property, value) = Property::parse_property("CONTACT;ALTREP=\"ldap://host.com:6666/o=3DABC%20Industries\\,c=3DUS??(cn=3DBJim%20Dolittle\":Jim Dolittle\\, ABC Industries\\,+1-919-555-1234".to_string()).unwrap();
    let (property, value) = Property::parse_property("CONTACT;ALTREP=\"CID=<part3.msg970930T083000SILVER@host.com>\":JimDolittle\\, ABC Industries\\, +1-919-555-1234".to_string()).unwrap();
    let (property, value) = Property::parse_property("CONTACT;ALTREP=\"http://host.com/pdi/jdoe.vcf\":JimDolittle\\, ABC Industries\\, +1-919-555-1234".to_string()).unwrap();
}

#[test]
fn geo_parsing_cases() {
    assert_eq!(
        Property::parse_property("GEO:92.386013;122.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected
    );
    assert_eq!(
        Property::parse_property("GEO:-92.386013;122.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected
    );
    assert_eq!(
        Property::parse_property("GEO:82.386013;192.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected
    );
    assert_eq!(
        Property::parse_property("GEO:82.386013;-192.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected
    );
}

#[test]
fn date_time_parsing_cases() {
    // Random bad value
    assert_eq!(
        Property::parse_property("DTSTAMP:QSDSD".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected
    );

    // Able to read date_time
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 3, 13)
        .unwrap()
        .and_hms_opt(12, 34, 32)
        .unwrap();

    let (_, value) = Property::parse_property("DTSTAMP:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 5, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let (_, value) = Property::parse_property("DUE;VALUE=DATE:20070501".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
}
