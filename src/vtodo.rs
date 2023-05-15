/*
Purpose:  Provide a grouping of calendar properties that describe a
      to-do.

   Format Definition:  A "VTODO" calendar component is defined by the
      following notation:

       todoc      = "BEGIN" ":" "VTODO" CRLF
                    todoprop *alarmc
                    "END" ":" "VTODO" CRLF

       todoprop   = *(
                  ;
                  ; The following are REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  dtstamp / uid /
                  ;
                  ; The following are OPTIONAL,
                  ; but MUST NOT occur more than once.
                  ;
                  class / completed / created / description /
                  dtstart / geo / last-mod / location / organizer /
                  percent / priority / recurid / seq / status /
                  summary / url /
                  ;
                  ; The following is OPTIONAL,
                  ; but SHOULD NOT occur more than once.
                  ;
                  rrule /
                  ;
                  ; Either 'due' or 'duration' MAY appear in
                  ; a 'todoprop', but 'due' and 'duration'
                  ; MUST NOT occur in the same 'todoprop'.
                  ; If 'duration' appear in a 'todoprop',
                  ; then 'dtstart' MUST also appear in
                  ; the same 'todoprop'.
                  ;
                  due / duration /
                  ;
                  ; The following are OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  attach / attendee / categories / comment / contact /
                  exdate / rstatus / related / resources /
                  rdate / x-prop / iana-prop
                  ;
                  )

   Description:  A "VTODO" calendar component is a grouping of component
      properties and possibly "VALARM" calendar components that
      represent an action-item or assignment.  For example, it can be
      used to represent an item of work assigned to an individual; such
      as "turn in travel expense today".

      The "VTODO" calendar component cannot be nested within another
      calendar component.  However, "VTODO" calendar components can be
      related to each other or to a "VEVENT" or to a "VJOURNAL" calendar
      component with the "RELATED-TO" property.

      A "VTODO" calendar component without the "DTSTART" and "DUE" (or
      "DURATION") properties specifies a to-do that will be associated
      with each successive calendar date, until it is completed.
*/

use crate::ics_error::ICSError;
use crate::properties::class::Class;
use crate::properties::uri::Uri;
use crate::properties::Property;
use crate::properties::{cal_adress::CalAdress, status::Status};
use crate::utils;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use std::fs::File;
use std::io::{BufReader, Lines};

#[cfg(test)]
use chrono::TimeZone;
#[cfg(test)]
use std::io::BufRead;

#[derive(Debug)]
pub struct VTodo {
    // Necessary variables
    pub dtstamp: DateTime<FixedOffset>,
    pub uid: String,

    // Optional and unique
    pub class: Option<Class>,
    pub completed: Option<DateTime<FixedOffset>>,
    pub created: Option<DateTime<FixedOffset>>,
    pub description: Option<String>,
    pub dtstart: Option<DateTime<FixedOffset>>,
    pub geo: Option<(f32, f32)>,
    pub last_modified: Option<DateTime<FixedOffset>>,
    pub location: Option<String>,
    pub organizer: Option<CalAdress>,
    pub percent: Option<usize>,
    pub priority: Option<usize>,
    pub recurrence_id: Option<DateTime<FixedOffset>>,
    pub sequence: Option<usize>,
    pub status: Option<Status>,
    pub summary: Option<String>,
    pub url: Option<Uri>,

    // Optional and conditional
    pub due: Option<DateTime<FixedOffset>>,
    pub duration: Option<Duration>,

    // Optional and several
    pub attach: Vec<Uri>,
    pub attendee: Vec<CalAdress>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<CalAdress>,
    pub exdate: Vec<DateTime<FixedOffset>>,
    // rstatus: Vec<String> // Seems to be a request answer so I wont be putting it in for now.
    pub related_to: Vec<String>,
    pub resources: Vec<String>,
    pub rdate: Vec<DateTime<FixedOffset>>,
    // x_prop: Will be implemented later
    // iana_prop: Will be implemented later
}

impl VTodo {
    pub fn new_empty(dtstamp: DateTime<FixedOffset>, uid: String) -> VTodo {
        VTodo {
            dtstamp,
            uid,
            class: None,
            completed: None,
            created: None,
            description: None,
            dtstart: None,
            geo: None,
            last_modified: None,
            location: None,
            organizer: None,
            percent: None,
            priority: None,
            recurrence_id: None,
            sequence: None,
            status: None,
            summary: None,
            url: None,
            due: None,
            duration: None,
            attach: Vec::new(),
            attendee: Vec::new(),
            categories: Vec::new(),
            comment: Vec::new(),
            contact: Vec::new(),
            exdate: Vec::new(),
            related_to: Vec::new(),
            resources: Vec::new(),
            rdate: Vec::new(),
        }
    }

    /// Reads the content of a VTODO object. The buffer passed should already have consumed the BEGIN:VTODO.
    pub fn parse_from_bufreader(
        line_reader: &mut Lines<BufReader<File>>,
    ) -> Result<VTodo, ICSError> {
        let mut vtodo: VTodo = VTodo::new_empty(
            DateTime::from_utc(
                Utc::now().naive_utc(),
                FixedOffset::east_opt(0).expect("FixedOffset::east out of bounds"),
            ),
            "".to_string(),
        );
        let mut has_uid = false;
        let mut has_dtstamp = false;

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
                    if processed_line.starts_with("END:VTODO") {
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
                Property::DTStamp => {
                    if has_dtstamp {
                        return Err(ICSError::DuplicateUniqueProperty(property_string));
                    }
                    has_dtstamp = true;
                    vtodo.dtstamp = value.try_into().unwrap();
                }
                Property::Completed => {
                    utils::apply_unique_property(&mut vtodo.completed, value, property_string)?
                }
                Property::Created => {
                    utils::apply_unique_property(&mut vtodo.created, value, property_string)?
                }
                Property::DTStart => {
                    utils::apply_unique_property(&mut vtodo.dtstart, value, property_string)?
                }
                Property::LastModified => {
                    utils::apply_unique_property(&mut vtodo.last_modified, value, property_string)?
                }
                Property::RecurrenceID => todo!(),
                Property::ExDate => vtodo.exdate.push(value.try_into().unwrap()),
                Property::RDate => vtodo.rdate.push(value.try_into().unwrap()),
                Property::Due => {
                    utils::apply_unique_property(&mut vtodo.due, value, property_string)?
                }
                Property::Duration => todo!(),
                Property::UID => {
                    if has_uid {
                        return Err(ICSError::DuplicateUniqueProperty(property_string));
                    }
                    has_uid = true;
                    vtodo.uid = value.try_into().unwrap();
                }
                Property::Description => {
                    utils::apply_unique_property(&mut vtodo.description, value, property_string)?
                }
                Property::Location => {
                    utils::apply_unique_property(&mut vtodo.location, value, property_string)?
                }
                Property::Summary => {
                    utils::apply_unique_property(&mut vtodo.summary, value, property_string)?
                }
                Property::Comment => vtodo.comment.push(value.try_into().unwrap()),
                Property::RelatedTo => vtodo.related_to.push(value.try_into().unwrap()),
                Property::Resources => vtodo.resources.push(value.try_into().unwrap()),
                Property::Categories => {
                    let mut string_vect: Vec<String> = value.try_into().unwrap();
                    vtodo.categories.append(&mut string_vect);
                }
                Property::Organizer => todo!(),
                Property::Attendee => todo!(),
                Property::Contact => todo!(),
                Property::PercentComplete => {
                    utils::apply_unique_property(&mut vtodo.percent, value, property_string)?
                }
                Property::Priority => {
                    utils::apply_unique_property(&mut vtodo.priority, value, property_string)?
                }
                Property::Sequence => {
                    utils::apply_unique_property(&mut vtodo.sequence, value, property_string)?
                }
                Property::Status => {
                    if vtodo.status.is_some() {
                        return Err(ICSError::DuplicateUniqueProperty(property_string));
                    }
                    let status: Status = value.try_into().unwrap();
                    if !status.validate_vtodo() {
                        return Err(ICSError::PropertyConditionNotRespected(property_string));
                    }
                    vtodo.status = Some(status);
                }
                Property::URL => todo!(),
                Property::Attach => todo!(),
                Property::Geo => {
                    utils::apply_unique_property(&mut vtodo.geo, value, property_string)?
                }
                Property::Class => {
                    utils::apply_unique_property(&mut vtodo.class, value, property_string)?
                }
                _ => return Err(ICSError::UnexpectedProperty(property_string)), // Other properties are not used
            }
        }

        if !has_uid {
            return Err(ICSError::MissingNecessaryProperty("UID".to_string()));
        }
        if !has_dtstamp {
            return Err(ICSError::MissingNecessaryProperty("DTSTAMP".to_string()));
        }

        Ok(vtodo)
    }
}

#[test]
fn vtodo_read_example_1() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_1").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    let vtodo = VTodo::parse_from_bufreader(&mut lines).unwrap();

    assert_eq!(vtodo.uid, "20070313T123432Z-456553@example.com");
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 3, 13, 12, 34, 32)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 5, 1, 0, 0, 0)
        .unwrap();
    assert_eq!(vtodo.due.unwrap(), expected_date);
    assert_eq!(
        vtodo.summary.unwrap(),
        "Submit Quebec Income Tax Return for 2006".to_string()
    );
    assert_eq!(vtodo.class.unwrap(), Class::CONFIDENTIAL);
    assert_eq!(vtodo.categories, vec!["FAMILY", "FINANCE"]);
    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
}

#[test]
fn vtodo_read_example_2() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_2").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    let vtodo = VTodo::parse_from_bufreader(&mut lines).unwrap();

    assert_eq!(vtodo.uid, "20070514T103211Z-123404@example.com");
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 5, 14, 10, 32, 11)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 5, 14, 11, 0, 0)
        .unwrap();
    assert_eq!(vtodo.dtstart.unwrap(), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2007, 7, 7, 10, 0, 0)
        .unwrap();
    assert_eq!(vtodo.completed.unwrap(), expected_date);

    assert_eq!(
        vtodo.summary.unwrap(),
        "Submit Revised Internet-Draft".to_string()
    );

    assert_eq!(
        vtodo.description.unwrap(),
        "This is a multi line description in order to test the multi line code".to_string()
    );

    assert_eq!(vtodo.priority.unwrap(), 1);
    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
}

#[test]
fn vtodo_read_example_3() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_3").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    let vtodo = VTodo::parse_from_bufreader(&mut lines).unwrap();

    assert_eq!(vtodo.uid, "19970901T130000Z-123404@host.com");

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(1997, 9, 1, 13, 0, 0)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(1997, 4, 15, 13, 30, 0)
        .unwrap();
    assert_eq!(vtodo.dtstart.unwrap(), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(1997, 4, 16, 4, 59, 59)
        .unwrap();
    assert_eq!(vtodo.due.unwrap(), expected_date);

    assert_eq!(vtodo.summary.unwrap(), "1996 Income Tax Preparation");
    assert_eq!(vtodo.class.unwrap(), Class::CONFIDENTIAL);
    assert_eq!(vtodo.categories, vec!["FAMILY", "FINANCE"]);
    assert_eq!(vtodo.priority.unwrap(), 1);
    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
}

/// THIS IS MISSING THE CAL ADRESSES AND URI
#[test]
fn vtodo_duplicate_variable() {
    let f = File::open("./tests/test_files/vtodo/duplicate_variable").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    let mut current_line = lines.next();

    let mut i: isize = 0;
    while let Some(ref non_null_line) = current_line {
        if non_null_line.as_ref().unwrap().as_str() == "BEGIN:VTODO" {
            i += 1;
            println!("Processing vtodo number {i}");
            let error = VTodo::parse_from_bufreader(&mut lines).unwrap_err();
            match error {
                ICSError::DuplicateUniqueProperty(_) => {}
                _ => panic!("Did not get a duplicate unique property"),
            }
            current_line = lines.next();
        } else {
            current_line = lines.next();
        }
    }
}
