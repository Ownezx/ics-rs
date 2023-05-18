/*
alarmc     = "BEGIN" ":" "VALARM" CRLF
                    (audioprop / dispprop / emailprop)
                    "END" ":" "VALARM" CRLF

       audioprop  = *(
                  ;
                  ; 'action' and 'trigger' are both REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  action / trigger /
                  ;
                  ; 'duration' and 'repeat' are both OPTIONAL,
                  ; and MUST NOT occur more than once each;
                  ; but if one occurs, so MUST the other.
                  ;
                  duration / repeat /
                  ;
                  ; The following is OPTIONAL,
                  ; but MUST NOT occur more than once.
                  ;
                  attach /
                  ;
                  ; The following is OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  x-prop / iana-prop
                  ;
                  )

       dispprop   = *(
                  ;
                  ; The following are REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  action / description / trigger /
                  ;
                  ; 'duration' and 'repeat' are both OPTIONAL,
                  ; and MUST NOT occur more than once each;
                  ; but if one occurs, so MUST the other.
                  ;
                  duration / repeat /
                  ;
                  ; The following is OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  x-prop / iana-prop
                  ;
                  )

       emailprop  = *(
                  ;
                  ; The following are all REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  action / description / trigger / summary /
                  ;
                  ; The following is REQUIRED,
                  ; and MAY occur more than once.
                  ;
                  attendee /
                  ;
                  ; 'duration' and 'repeat' are both OPTIONAL,
                  ; and MUST NOT occur more than once each;
                  ; but if one occurs, so MUST the other.
                  ;
                  duration / repeat /
                  ;
                  ; The following are OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  attach / x-prop / iana-prop
                  ;
                  )
 */

use crate::ics_error::ICSError;
use crate::properties::action::Action;
use crate::properties::uri::Uri;
use crate::properties::Property;
use crate::utils;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use std::fs::File;
use std::io::{BufReader, Lines};

#[cfg(test)]
use chrono::TimeZone;
#[cfg(test)]
use std::io::BufRead;

#[derive(Debug)]
pub struct VAlarm {
    // Necessary variables
    pub action: Action,
    pub trigger: String, // Need to create it's own value

    // Sometimes necessary variable
    pub summary: Option<String>,
    pub description: Option<String>,

    // Optional and conditional
    pub duration: Option<Duration>,
    pub repeat: Option<usize>,

    // This has different possibilities depending on the type of Valarm
    pub attach: Vec<Uri>,
    // xprop, iana prop
}

impl VAlarm {
    pub fn new_empty(action: Action, trigger: String) -> VAlarm {
        VAlarm {
            action,
            trigger,
            description: None,
            summary: None,
            duration: None,
            repeat: None,
            attach: Vec::new(),
        }
    }

    /// Reads the content of a VTODO object. The buffer passed should already have consumed the BEGIN:VTODO.
    pub fn parse_from_bufreader(
        line_reader: &mut Lines<BufReader<File>>,
    ) -> Result<VAlarm, ICSError> {
        let mut vtodo: VAlarm = VAlarm::new_empty(Action::Display, "".to_string());
        let mut has_action = false;
        let mut has_trigger = false;

        let mut current_line: Option<Result<String, std::io::Error>> = line_reader.next();

        loop {
            let line = current_line;
            let processed_line: String;
            match line {
                Some(line) => {
                    // Read line
                    processed_line = match line {
                        Ok(val) => val,
                        Err(_) => return Err(ICSError::ReadError),
                    };
                    // End the process if we have arrived at the end.
                    if processed_line.starts_with("END:VALARM") {
                        break;
                    }
                }
                None => return Err(ICSError::BeginWithoutEnd),
            }

            // Here we need to be able to process multi line arguments.
            let property_string: String;
            (property_string, current_line) =
                utils::process_multi_line_property(processed_line, line_reader);

            // I clone the line here to avoid borrowing it as I might give it to an error.
            // This is probably slow but let's leave that problem for future smarter me.
            let (property, value) = Property::parse_property(property_string.clone())?;

            match property {
                Property::Duration => todo!(),
                Property::Description => todo!(),
                Property::Summary => todo!(),
                Property::Action => todo!(),
                Property::URL => todo!(),
                Property::Attach => todo!(),
                Property::Trigger => todo!(),
                Property::Repeat => todo!(),
                _ => return Err(ICSError::UnexpectedProperty(property_string)), // Other properties are not used
            }
        }

        if !has_action {
            return Err(ICSError::MissingNecessaryProperty("ACTION".to_string()));
        }
        if !has_trigger {
            return Err(ICSError::MissingNecessaryProperty("TRIGGER".to_string()));
        }

        Ok(vtodo)
    }
}

#[ignore = "Not implemented yet"]
#[test]
fn valarm_read_example_1() {
    let f = File::open("./tests/test_files/valarm/example1").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    //let valarm = VAlarm::parse_from_bufreader(&mut lines).unwrap();
}

#[ignore = "Not implemented yet"]
#[test]
fn valarm_read_example_2() {
    let f = File::open("./tests/test_files/valarm/example2").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    //let valarm = VAlarm::parse_from_bufreader(&mut lines).unwrap();
}

#[ignore = "Not implemented yet"]
#[test]
fn valarm_read_example_3() {
    let f = File::open("./tests/test_files/valarm/example3").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    //let valarm = VAlarm::parse_from_bufreader(&mut lines).unwrap();
}

#[ignore = "Not implemented yet"]
#[test]
fn valarm_read_example_4() {
    let f = File::open("./tests/test_files/valarm/example4").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    //let valarm = VAlarm::parse_from_bufreader(&mut lines).unwrap();
}
