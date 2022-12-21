/*
The body of the iCalendar object consists of a sequence of calendar
properties and one or more calendar components. The calendar
properties are attributes that apply to the calendar as a whole. The
calendar components are collections of properties that express a
particular calendar semantic. For example, the calendar component can
specify an event, a to-do, a journal entry, time zone information, or
free/busy time information, or an alarm.

The body of the iCalendar object is defined by the following
notation:

  icalbody   = calprops component

  calprops   = 2*(

             ; 'prodid' and 'version' are both REQUIRED,
             ; but MUST NOT occur more than once

             prodid /version /

             ; 'calscale' and 'method' are optional,
             ; but MUST NOT occur more than once

             calscale        /
             method          /

             x-prop
             )

  component  = 1*(eventc / todoc / journalc / freebusyc /
             / timezonec / iana-comp / x-comp)

  iana-comp  = "BEGIN" ":" iana-token CRLF

               1*contentline

               "END" ":" iana-token CRLF

  x-comp     = "BEGIN" ":" x-name CRLF

               1*contentline

               "END" ":" x-name CRLF

An iCalendar object MUST include the "PRODID" and "VERSION" calendar
properties. In addition, it MUST include at least one calendar
component. Special forms of iCalendar objects are possible to publish
just busy time (i.e., only a "VFREEBUSY" calendar component) or time
zone (i.e., only a "VTIMEZONE" calendar component) information. In
addition, a complex iCalendar object is possible that is used to
capture a complete snapshot of the contents of a calendar (e.g.,
composite of many different calendar components). More commonly, an
iCalendar object will consist of just a single "VEVENT", "VTODO" or
"VJOURNAL" calendar component.
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::ics_error::ICSError;

use crate::properties::Property;
use crate::utils;
use crate::vevent::VEvent;
use crate::vjournal::VJournal;
use crate::vtodo::VTodo;

#[cfg(test)]
use crate::properties::status::Status;
#[cfg(test)]
use chrono::{FixedOffset, TimeZone};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct VCalendar {
    // Necessary variables
    prodid: String,
    version: String,

    // Optional variables
    calscale: Option<String>,
    method: Option<String>,

    // One of the components
    vjournal: Option<VJournal>,
    vtodo: Option<VTodo>,
    vevent: Option<VEvent>,
}

impl VCalendar {
    pub fn new_empty() -> VCalendar {
        VCalendar {
            prodid: format!("-//ics-rs//{VERSION}//EN"),
            version: "2.0".to_string(),
            calscale: None,
            method: None,
            vjournal: None,
            vtodo: None,
            vevent: None,
        }
    }

    pub fn load_vcal_from_file(path: &Path) -> Result<VCalendar, ICSError> {
        match path.extension() {
            Some(ext_value) => {
                if ext_value != "ics" {
                    return Err(ICSError::NotICSFile);
                }
            }
            None => return Err(ICSError::NotICSFile),
        }

        let mut has_prod_id = false;
        let mut has_version = false;

        let f = File::open(path).unwrap();
        let buf_reader = BufReader::new(f);
        let mut line_reader = buf_reader.lines();
        let mut vcal_object = VCalendar::new_empty();

        // Find first BEGIN:VCALENDAR
        match line_reader.next() {
            Some(result) => match result {
                Ok(line) => line,
                Err(_) => return Err(ICSError::ReadError),
            },
            None => return Err(ICSError::NoBegin),
        };

        let mut current_line: Option<Result<String, std::io::Error>> = line_reader.next();

        loop {
            let line = current_line;
            let processed_line: String;
            match line {
                Some(line) => {
                    // Read line
                    processed_line = line.unwrap();
                    // End the process if we have arrived at the end.
                    if processed_line.starts_with("END:VCALENDAR") {
                        break;
                    }
                }
                None => return Err(ICSError::BeginWithoutEnd),
            }
            // Here we need to be able to process multi line arguments.

            if processed_line.starts_with("BEGIN") {
                let begin_val: &str = match processed_line.split_once(':') {
                    Some((_, l)) => l,
                    None => return Err(ICSError::InvalidBeginLine(processed_line)),
                };

                match begin_val {
                    "VTODO" => {
                        if vcal_object.vtodo.is_some() {
                            return Err(ICSError::DuplicateUniqueProperty(begin_val.to_string()));
                        }
                        vcal_object.vtodo = Some(VTodo::parse_from_bufreader(&mut line_reader)?);
                    }
                    "VEVENT" => {
                        if vcal_object.vevent.is_some() {
                            return Err(ICSError::DuplicateUniqueProperty(begin_val.to_string()));
                        }
                        vcal_object.vevent = Some(VEvent::parse_from_bufreader(&mut line_reader)?);
                    }
                    "VJOURNAL" => {
                        if vcal_object.vjournal.is_some() {
                            return Err(ICSError::DuplicateUniqueProperty(begin_val.to_string()));
                        }
                        vcal_object.vjournal =
                            Some(VJournal::parse_from_bufreader(&mut line_reader)?);
                    }
                    _ => return Err(ICSError::UnknownComponent(begin_val.to_string())),
                }

                // Consume next line as we have finished the VTODO
                current_line = line_reader.next();
                continue;
            }

            let property_string: String;
            (property_string, current_line) =
                utils::process_multi_line_property(processed_line, &mut line_reader);

            // I clone the line here to avoid borrowing it as I might give it to an error.
            // This is probably slow but let's leave that problem for future smarter me.
            let (property, value) = Property::parse_property(property_string.clone())?;
            match property {
                Property::ProdID => {
                    if has_prod_id {
                        return Err(ICSError::DuplicateUniqueProperty(property_string));
                    }
                    has_prod_id = true;
                    vcal_object.prodid = value.try_into().unwrap();
                }
                Property::Version => {
                    if has_version {
                        return Err(ICSError::DuplicateUniqueProperty(property_string));
                    }
                    has_version = true;
                    vcal_object.version = value.try_into().unwrap();
                }
                Property::CalScale => {
                    utils::apply_unique_property(&mut vcal_object.calscale, value, property_string)?
                }
                Property::Method => {
                    utils::apply_unique_property(&mut vcal_object.method, value, property_string)?
                }
                _ => return Err(ICSError::UnexpectedProperty(property_string)), // Other properties are not used
            }
        }

        // Verify duplicate property
        match (
            &vcal_object.vevent,
            &vcal_object.vjournal,
            &vcal_object.vtodo,
        ) {
            (None, None, Some(_)) => {}
            (None, Some(_), None) => {}
            (Some(_), None, None) => {}
            (None, None, None) => {
                return Err(ICSError::MissingNecessaryProperty(
                    "VTODO, VCALENDAR, VJOURNAL".to_string(),
                ))
            }
            (_, _, _) => {
                return Err(ICSError::DuplicateUniqueProperty(
                    "VTODO, VCALENDAR, VJOURNAL".to_string(),
                ))
            }
        }

        if !has_prod_id {
            return Err(ICSError::MissingNecessaryProperty("PRODID".to_string()));
        }
        if !has_version {
            return Err(ICSError::MissingNecessaryProperty("VERSION".to_string()));
        }

        Ok(vcal_object)
    }
}

#[test]
fn ics_extention_verification() {
    assert_eq!(
        VCalendar::load_vcal_from_file(Path::new("test.random")).unwrap_err(),
        ICSError::NotICSFile
    );

    assert_eq!(
        VCalendar::load_vcal_from_file(Path::new("test.icsr")).unwrap_err(),
        ICSError::NotICSFile
    );

    assert_eq!(
        VCalendar::load_vcal_from_file(Path::new("testics")).unwrap_err(),
        ICSError::NotICSFile
    );
}

#[test]
fn vtodo_example_1() {
    let vcal_object =
        VCalendar::load_vcal_from_file(Path::new("./tests/test_files/vtodo/example2.ics")).unwrap();

    let vtodo = vcal_object.vtodo.unwrap();

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2022, 11, 17, 19, 55, 32)
        .unwrap();
    assert_eq!(vtodo.created.unwrap(), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2022, 11, 17, 19, 55, 38)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2022, 11, 17, 19, 55, 32)
        .unwrap();
    assert_eq!(vtodo.last_modified.unwrap(), expected_date);

    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
    assert_eq!(vtodo.summary.unwrap(), "test".to_string());

    assert_eq!(
        vtodo.uid,
        "F01AAFD6-686E-4FD7-8A94-9D7EB876A6F6".to_string()
    );
}

#[ignore]
#[test]
fn missing_properties() {
    // No VTODO,VEVENT,VCALENDAR
    // No ProdID
    // No Version
    todo!();
}

#[ignore]
#[test]
fn duplicate_unique_properties() {
    // Two VTODO, VEVENT, VCALENDAR
    // One VTODO and andother ... Etc
    todo!();
}

#[ignore]
#[test]
fn x_components_tests() {
    todo!();
}

#[ignore]
#[test]
fn iana_token_components_tests() {
    todo!();
}
