use std::{ops::Add, str::FromStr};

#[cfg(test)]
use chrono::TimeZone;
use chrono::{DateTime, Duration, FixedOffset};

use crate::ics_error::ICSError;

use self::{action::Action, class::Class, status::Status};

pub mod action;
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
    "PRODID",
    "VERSION",
    "CALSCALE",
    "METHOD",
    // Cal address
    "ORGANIZER",
    "ATTENDEE",
    "CONTACT",
    // Integer
    "PERCENT-COMPLETE",
    "PRIORITY",
    "SEQUENCE",
    "REPEAT",
    // Status
    "STATUS",
    // Action
    "ACTION",
    // URI
    "URL",
    "ATTACH",
    // Others
    "GEO",
    "CLASS",
    "TRIGGER",
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
    ProdID,
    Version,
    CalScale,
    Method,

    // Cal adress properties
    Organizer,
    Attendee,
    Contact,

    // Integer properties
    PercentComplete,
    Priority,
    Sequence,
    Repeat,

    // Status,
    Status,

    // Status,
    Action,

    // URI properties
    URL,
    Attach,

    // Others
    Geo,
    Class,
    Trigger,
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
            None => return Err(ICSError::UnableToParseProperty(line)),
        };
        let mut parameters = splitted_line.0.split(';');

        let property_name = parameters.next().unwrap();
        // println!("{}",var);
        let property = Property::get_property_from_identifier(property_name);

        if property.is_none() {
            return Err(ICSError::UknownProperty(property_name.to_string()));
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
                        None => {
                            return Err(ICSError::PropertyConditionNotRespected(
                                property_name.to_string(),
                            ))
                        }
                    };

                    // Match the parameter with different possibilities
                    match param_name {
                        "VALUE" => {
                            match param_value {
                                // If it is a date, lets add some 0 time to parse it properly
                                "DATE" => temp_string.push_str("T000000Z"),
                                "DATE-TIME" => {}
                                _ => {
                                    return Err(ICSError::PropertyConditionNotRespected(
                                        property_name.to_string(),
                                    ))
                                }
                            }
                        }
                        _ => {
                            return Err(ICSError::PropertyConditionNotRespected(
                                property_name.to_string(),
                            ))
                        }
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
                                Err(_) => {
                                    return Err(ICSError::PropertyConditionNotRespected(
                                        property_name.to_string(),
                                    ))
                                }
                            }
                        }
                    };
                ParserResult::DateTime(date_time)
            }
            // Duration property
            Property::Duration => {
                // Because the duration cannot include months or years
                // it's analog to a duration in time
                let mut temp_string = String::from(splitted_line.1);
                // Create are 0 duration before adding more to it.
                let mut duration: Duration = Duration::days(0);

                let mut factor: i64 = 1;

                // Try to the negative
                let split = temp_string.split_once('P');
                // verify that the start of the string is correct
                match split {
                    Some(vec) => {
                        match (!vec.0.is_empty(), vec.0.starts_with('-')) {
                            // We are negative
                            (true, true) => factor = -1,
                            // We are starting with the wrong character
                            (true, false) => {
                                return Err(ICSError::PropertyConditionNotRespected(
                                    property_name.to_string(),
                                ))
                            }
                            (_, _) => {}
                        }
                        temp_string = vec.1.to_string();
                    }
                    None => {
                        return Err(ICSError::PropertyConditionNotRespected(
                            property_name.to_string(),
                        ))
                    }
                }

                // Try to find week
                let split = temp_string.split_once('W');
                // Add it if it's there
                if let Some(vec) = split {
                    duration = duration.add(Duration::weeks(
                        factor
                            * <i32 as Into<i64>>::into(vec.0.to_string().parse::<i32>().unwrap()),
                    ));
                    temp_string = vec.1.to_string();
                }

                // Try to find days
                let split = temp_string.split_once('D');
                // Add it if it's there
                if let Some(vec) = split {
                    duration = duration.add(Duration::days(
                        factor
                            * <i32 as Into<i64>>::into(vec.0.to_string().parse::<i32>().unwrap()),
                    ));
                    temp_string = vec.1.to_string();
                }

                // Try to find A time
                let split = temp_string.split_once('T');
                // Add it if it's there
                if let Some(vec) = split {
                    temp_string = vec.1.to_string();

                    // Try to find hours
                    let split = temp_string.split_once('H');
                    // Add it if it's there
                    if let Some(vec) = split {
                        duration = duration.add(Duration::hours(
                            factor
                                * <i32 as Into<i64>>::into(
                                    vec.0.to_string().parse::<i32>().unwrap(),
                                ),
                        ));
                        temp_string = vec.1.to_string();
                    }

                    // Try to find minutes
                    let split = temp_string.split_once('M');
                    // Add it if it's there
                    if let Some(vec) = split {
                        duration = duration.add(Duration::minutes(
                            factor
                                * <i32 as Into<i64>>::into(
                                    vec.0.to_string().parse::<i32>().unwrap(),
                                ),
                        ));
                        temp_string = vec.1.to_string();
                    }

                    // Try to find seconds
                    let split = temp_string.split_once('S');
                    // Add it if it's there
                    if let Some(vec) = split {
                        duration = duration.add(Duration::seconds(
                            factor
                                * <i32 as Into<i64>>::into(
                                    vec.0.to_string().parse::<i32>().unwrap(),
                                ),
                        ));
                        temp_string = vec.1.to_string();
                    }
                }

                // Verify that the string is completely eaten
                if !temp_string.is_empty() {
                    return Err(ICSError::PropertyConditionNotRespected(
                        property_name.to_string(),
                    ));
                }

                ParserResult::Duration(duration)
            }
            // String identifier
            // We might want to add a specific validator for UID
            Property::UID
            | Property::Trigger
            | Property::Description
            | Property::Location
            | Property::Summary
            | Property::Comment
            | Property::RelatedTo
            | Property::Resources
            | Property::ProdID
            | Property::Version
            | Property::CalScale
            | Property::Method => ParserResult::String(String::from(splitted_line.1)),

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

            Property::PercentComplete
            | Property::Repeat
            | Property::Priority
            | Property::Sequence => match splitted_line.1.to_string().parse() {
                Ok(integer) => ParserResult::Integer(integer),
                Err(_) => return Err(ICSError::UnableToParseProperty(property_name.to_string())),
            },

            Property::Status => ParserResult::Status(Status::from_str(splitted_line.1)?),

            Property::Action => ParserResult::Action(Action::from_str(splitted_line.1)?),

            Property::URL | Property::Attach => todo!(),

            Property::Geo => {
                // Get the two floats
                let (lat, long) = match splitted_line.1.split_once(';') {
                    Some(values) => values,
                    None => return Err(ICSError::UnableToParseProperty(property_name.to_string())),
                };
                let float_lat: f32 = match lat.to_string().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        return Err(ICSError::UnableToParseProperty(property_name.to_string()))
                    }
                };
                let float_long: f32 = match long.to_string().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        return Err(ICSError::UnableToParseProperty(property_name.to_string()))
                    }
                };

                if !(-90. ..=90.).contains(&float_lat) {
                    return Err(ICSError::PropertyConditionNotRespected(
                        property_name.to_string(),
                    ));
                }

                if !(-180. ..=180.).contains(&float_long) {
                    return Err(ICSError::PropertyConditionNotRespected(
                        property_name.to_string(),
                    ));
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
    Action(Action),
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

impl From<ParserResult> for Action {
    fn from(result: ParserResult) -> Self {
        match result {
            ParserResult::Action(val) => val,
            _ => panic!("Not casting the right result"),
        }
    }
}

#[test]
fn all_properties_properly_recognised() {
    // Date/Datetime
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 3, 13, 12, 34, 32)
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

    // Duration
    let (property, value) = Property::parse_property("DURATION:P1W".to_string()).unwrap();
    assert_eq!(Duration::from(value), Duration::weeks(1));
    assert_eq!(property, Property::Duration);

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
        Property::parse_property("PRODID:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::ProdID);

    let (property, value) =
        Property::parse_property("VERSION:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Version);

    let (property, value) =
        Property::parse_property("CALSCALE:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::CalScale);

    let (property, value) =
        Property::parse_property("METHOD:This is a description".to_string()).unwrap();
    assert_eq!(String::from(value), "This is a description".to_string());
    assert_eq!(property, Property::Method);

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

    // Action
    let (property, value) = Property::parse_property("ACTION:DISPLAY".to_string()).unwrap();
    assert_eq!(Action::from(value), Action::Display);
    assert_eq!(property, Property::Action);

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
    assert_eq!(result, Err(ICSError::UknownProperty("SDQ".to_string())));
}

#[ignore = "Not implemented yet"]
#[test]
fn wrong_calscale() {
    //let (property, value) = Property::parse_property("CALSCALE:Wrong".to_string()).unwrap();
}

#[ignore = "Not implemented yet"]
#[test]
fn cal_address_parsing_cases() {
    // let (property, value) =
    //     Property::parse_property("ORGANIZER:MAILTO:jane_doe@host.com".to_string()).unwrap();
    // let (property, value) =
    //     Property::parse_property("ORGANIZER;CN=John Smith:MAILTO:jsmith@host1.com".to_string())
    //         .unwrap();
    // let (property, value) = Property::parse_property(
    //     "ORGANIZER;CN=JohnSmith;DIR=\"ldap://host.com:6666/o=3DDC%20Associ
    // ates,c=3DUS??(cn=3DJohn%20Smith)\":MAILTO:jsmith@host1.com"
    //         .to_string(),
    // )
    // .unwrap();
    // let (property, value) = Property::parse_property(
    //     "ORGANIZER;SENT-BY=\"MAILTO:jane_doe@host.com\":MAILTO:jsmith@host1.com".to_string(),
    // )
    // .unwrap();
    // let (property, value) = Property::parse_property(
    //     "CONTACT:Jim Dolittle\\, ABC Industries\\, +1-919-555-1234".to_string(),
    // )
    // .unwrap();
    // let (property, value) = Property::parse_property("CONTACT;ALTREP=\"ldap://host.com:6666/o=3DABC%20Industries\\,c=3DUS??(cn=3DBJim%20Dolittle\":Jim Dolittle\\, ABC Industries\\,+1-919-555-1234".to_string()).unwrap();
    // let (property, value) = Property::parse_property("CONTACT;ALTREP=\"CID=<part3.msg970930T083000SILVER@host.com>\":JimDolittle\\, ABC Industries\\, +1-919-555-1234".to_string()).unwrap();
    // let (property, value) = Property::parse_property("CONTACT;ALTREP=\"http://host.com/pdi/jdoe.vcf\":JimDolittle\\, ABC Industries\\, +1-919-555-1234".to_string()).unwrap();
}

#[test]
fn geo_parsing_cases() {
    assert_eq!(
        Property::parse_property("GEO:92.386013;122.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("GEO".to_string())
    );
    assert_eq!(
        Property::parse_property("GEO:-92.386013;122.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("GEO".to_string())
    );
    assert_eq!(
        Property::parse_property("GEO:82.386013;192.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("GEO".to_string())
    );
    assert_eq!(
        Property::parse_property("GEO:82.386013;-192.082932".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("GEO".to_string())
    );
}

#[test]
fn duration_parsing_cases() {
    let (property, value) = Property::parse_property("DURATION:P15DT5H0M20S".to_string()).unwrap();
    assert_eq!(
        Duration::from(value),
        Duration::seconds(15 * 24 * 60 * 60 + 5 * 60 * 60 + 20)
    );
    assert_eq!(property, Property::Duration);

    let (property, value) = Property::parse_property("DURATION:P7W".to_string()).unwrap();
    assert_eq!(Duration::from(value), Duration::weeks(7));
    assert_eq!(property, Property::Duration);

    let (property, value) = Property::parse_property("DURATION:PT1H0M0S".to_string()).unwrap();
    assert_eq!(Duration::from(value), Duration::hours(1));
    assert_eq!(property, Property::Duration);

    let (property, value) = Property::parse_property("DURATION:PT15M".to_string()).unwrap();
    assert_eq!(Duration::from(value), Duration::minutes(15));
    assert_eq!(property, Property::Duration);

    let (property, value) = Property::parse_property("DURATION:-PT15M".to_string()).unwrap();
    assert_eq!(Duration::from(value), Duration::minutes(-15));
    assert_eq!(property, Property::Duration);

    // Bad first character
    assert_eq!(
        Property::parse_property("DURATION:DPT15M".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("DURATION".to_string())
    );
    assert_eq!(
        Property::parse_property("DURATION:-PJ".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("DURATION".to_string())
    );
}

#[test]
fn action_parsing_cases() {
    assert_eq!(
        Action::from(
            Property::parse_property("ACTION:DISPLAY".to_string())
                .unwrap()
                .1
        ),
        Action::Display
    );

    assert_eq!(
        Action::from(
            Property::parse_property("ACTION:EMAIL".to_string())
                .unwrap()
                .1
        ),
        Action::Email
    );

    assert_eq!(
        Action::from(
            Property::parse_property("ACTION:AUDIO".to_string())
                .unwrap()
                .1
        ),
        Action::Audio
    );
}

#[test]
fn date_time_parsing_cases() {
    // Random bad value
    assert_eq!(
        Property::parse_property("DTSTAMP:QSDSD".to_string()).unwrap_err(),
        ICSError::PropertyConditionNotRespected("DTSTAMP".to_string())
    );

    // Able to read date_time
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 3, 13, 12, 34, 32)
        .unwrap();

    let (_, value) = Property::parse_property("DTSTAMP:20070313T123432Z".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 5, 1, 0, 0, 0)
        .unwrap();
    let (_, value) = Property::parse_property("DUE;VALUE=DATE:20070501".to_string()).unwrap();
    assert_eq!(DateTime::<FixedOffset>::from(value), expected_date);
}

#[ignore = "Not implemented yet"]
#[test]
fn trigger_parsing_cases() {
    todo!();
}

#[ignore = "Not implemented yet"]
#[test]
fn x_property_parsing_cases() {
    todo!();
}

#[ignore = "Not implemented yet"]
#[test]
fn iana_token_parse_cases() {
    todo!();
}
