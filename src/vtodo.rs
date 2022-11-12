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
use crate::properties::{cal_adress::CalAdress, status::VTodoStatus};
use crate::properties::{ParserResult, Property};
use crate::utils;
use chrono::{Date, DateTime, Duration, FixedOffset, Utc};
use std::fs::File;
use std::io::{BufReader, Lines};

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
    pub geo: Option<(f64, f64)>,
    pub last_modified: Option<DateTime<FixedOffset>>,
    pub location: Option<String>,
    pub organizer: Option<CalAdress>,
    pub percent: Option<isize>,
    pub priority: Option<isize>,
    pub recurrence_id: Option<DateTime<FixedOffset>>,
    pub sequence: Option<isize>,
    pub status: Option<VTodoStatus>,
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
        mut line_reader: Lines<BufReader<File>>,
    ) -> Result<VTodo, ICSError> {
        let mut vtodo: VTodo = VTodo::new_empty(
            DateTime::from_utc(Utc::now().naive_utc(), FixedOffset::east(0)),
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
                    processed_line = line.unwrap();
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
                utils::process_multi_line_property(processed_line, &mut line_reader);

            let (property, value) = Property::parse_property(property_string)?;

            match property {
                Property::DTStamp => {
                    if has_dtstamp {
                        return Err(ICSError::DuplicateUniqueProperty);
                    }
                    has_dtstamp = true;
                    vtodo.dtstamp = value.try_into().unwrap();
                }
                Property::Completed => todo!(),
                Property::Created => todo!(),
                Property::DTStart => todo!(),
                Property::LastModified => todo!(),
                Property::RecurrenceID => todo!(),
                Property::ExDate => todo!(),
                Property::RDate => todo!(),
                Property::Due => todo!(),
                Property::Duration => todo!(),
                Property::UID => {
                    if has_uid {
                        return Err(ICSError::DuplicateUniqueProperty);
                    }
                    has_uid = true;
                    vtodo.uid = value.try_into().unwrap();
                }
                Property::Description => todo!(),
                Property::Location => todo!(),
                Property::Summary => todo!(),
                Property::Comment => todo!(),
                Property::RelatedTo => todo!(),
                Property::Resources => todo!(),
                Property::Categories => todo!(),
                Property::Organizer => todo!(),
                Property::Attendee => todo!(),
                Property::Contact => todo!(),
                Property::PercentComplete => todo!(),
                Property::Priority => todo!(),
                Property::Sequence => todo!(),
                Property::Status => todo!(),
                Property::URL => todo!(),
                Property::Attach => todo!(),
                Property::Geo => todo!(),
                Property::Class => todo!(),
            }
        }

        if !has_uid || !has_dtstamp {
            return Err(ICSError::MissingNecessaryProperty);
        }

        Ok(vtodo)
    }
}
