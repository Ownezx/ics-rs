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
        let mut current_line = match line_reader.next() {
            Some(result) => match result {
                Ok(line) => line,
                Err(e) => return Err(ICSError::ReadError),
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
            let property_string: String;
            (property_string, current_line) =
                utils::process_multi_line_property(processed_line, &mut line_reader);

            if property_string.starts_with("BEGIN") {
                let begin_val: &str = match property_string.split_once(':') {
                    Some((_, l)) => l,
                    None => return Err(ICSError::InvalidBeginLine(property_string)),
                };

                match begin_val {
                    "VTODO" => {
                        match (
                            &vcal_object.vevent,
                            &vcal_object.vjournal,
                            &vcal_object.vtodo,
                        ) {
                            (None, None, None) => {
                                vcal_object.vtodo =
                                    Some(VTodo::parse_from_bufreader(&mut line_reader)?)
                            }
                            // You should only have on Component in a VCALENDAR
                            _ => {
                                return Err(ICSError::DuplicateUniqueProperty(
                                    begin_val.to_string(),
                                ))
                            }
                        }
                    }
                    "VEVENT" => {
                        match (
                            &vcal_object.vevent,
                            &vcal_object.vjournal,
                            &vcal_object.vtodo,
                        ) {
                            (None, None, None) => {
                                vcal_object.vevent =
                                    Some(VEvent::parse_from_bufreader(&mut line_reader)?)
                            }
                            // You should only have on Component in a VCALENDAR
                            _ => {
                                return Err(ICSError::DuplicateUniqueProperty(
                                    begin_val.to_string(),
                                ))
                            }
                        }
                    }
                    "VJOURNAL" => {
                        match (
                            &vcal_object.vevent,
                            &vcal_object.vjournal,
                            &vcal_object.vtodo,
                        ) {
                            (None, None, None) => {
                                vcal_object.vjournal =
                                    Some(VJournal::parse_from_bufreader(&mut line_reader)?)
                            }
                            // You should only have on Component in a VCALENDAR
                            _ => {
                                return Err(ICSError::DuplicateUniqueProperty(
                                    begin_val.to_string(),
                                ))
                            }
                        }
                    }

                    _ => return Err(ICSError::UnknownComponent(begin_val.to_string())),
                }
            }

            let (property, value) = Property::parse_property(property_string)?;
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
fn vtodo_parse_validation() {
    let vcal_object =
        VCalendar::load_vcal_from_file(Path::new("./tests/test_files/vtodo/example2.ics")).unwrap();
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
