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

use crate::properties::class::Class;
use crate::properties::uri::Uri;
use crate::properties::{cal_adress::CalAdress, status::VTodoStatus};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::BufReader;

pub struct VTodo {
    // Necessary variables
    dtstamp: DateTime<Utc>,
    uid: String,

    // Optional and unique
    class: Option<Class>,
    completed: Option<DateTime<Utc>>,
    created: Option<DateTime<Utc>>,
    description: Option<String>,
    dtstart: Option<DateTime<Utc>>,
    geo: Option<(f64, f64)>,
    last_mod: Option<DateTime<Utc>>,
    location: Option<String>,
    organizer: Option<CalAdress>,
    percent: Option<isize>,
    priority: Option<isize>,
    recurrence_id: Option<DateTime<Utc>>,
    sequence: Option<isize>,
    status: Option<VTodoStatus>,
    summary: Option<String>,
    url: Option<Uri>,

    // Optional and several
    attach: Vec<Uri>,
    attendee: Vec<CalAdress>,
    categories: Vec<String>,
    comment: Vec<String>,
    contact: Vec<CalAdress>,
    exdate: Vec<DateTime<Utc>>,
    // rstatus: Vec<String> // Seems to be a request answer so I wont be putting it in for now.
    related: Vec<String>,
    resources: Vec<String>,
    rdate: Vec<DateTime<Utc>>,
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
            last_mod: None,
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
            related: Vec::new(),
            resources: Vec::new(),
            rdate: Vec::new(),
        }
    }

    pub fn parse_from_bufreader(reader: BufReader<File>) -> VTodo {
        let mut vtodo: VTodo = VTodo::new_empty(Utc::now(), "".to_string());
        let mut has_uid = false;
        let mut has_dtstamp = false;

        vtodo
    }
}
