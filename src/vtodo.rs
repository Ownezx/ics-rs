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
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct VTodo {
    // Necessary variables
    pub dtstamp: DateTime<Utc>,
    pub uid: String,

    // Optional and unique
    pub class: Option<Class>,
    pub completed: Option<DateTime<Utc>>,
    pub created: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub dtstart: Option<DateTime<Utc>>,
    pub geo: Option<(f64, f64)>,
    pub last_modified: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub organizer: Option<CalAdress>,
    pub percent: Option<isize>,
    pub priority: Option<isize>,
    pub recurrence_id: Option<DateTime<Utc>>,
    pub sequence: Option<isize>,
    pub status: Option<VTodoStatus>,
    pub summary: Option<String>,
    pub url: Option<Uri>,

    // Optional and several
    pub attach: Vec<Uri>,
    pub attendee: Vec<CalAdress>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<CalAdress>,
    pub exdate: Vec<DateTime<Utc>>,
    // rstatus: Vec<String> // Seems to be a request answer so I wont be putting it in for now.
    pub related_to: Vec<String>,
    pub resources: Vec<String>,
    pub rdate: Vec<DateTime<Utc>>,
    // x_prop: Will be implemented later
    // iana_prop: Will be implemented later
}

impl VTodo {
    pub fn new_empty(dtstamp: DateTime<Utc>, uid: String) -> VTodo {
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
    pub fn parse_from_bufreader(mut lines: Lines<BufReader<File>>) -> Result<VTodo, ICSError> {
        let mut vtodo: VTodo = VTodo::new_empty(Utc::now(), "".to_string());
        let mut has_uid = false;
        let mut has_dtstamp = false;

        loop {
            let line = lines.next();
            let processed_line: String;
            match line {
                Some(line) => {
                    //
                    processed_line = line.unwrap();

                    if processed_line.starts_with("END:VTODO") {
                        break;
                    }
                }
                None => return Err(ICSError::BeginWithoutEnd),
            }
        }

        if !has_uid || !has_dtstamp {
            return Err(ICSError::MissingNecessaryProperty);
        }

        Ok(vtodo)
    }
}
