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
use std::io::BufReader;
use std::path::Path;

use chrono::{DateTime, FixedOffset, TimeZone};
use chrono_tz::Tz;

use crate::ics_error::ICSError;
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
        if !path.ends_with(".ics") {
            return Err(ICSError::NotICSFile);
        }

        let f = File::open(path).unwrap();
        let buf_reader = BufReader::new(f);

        let mut vcal_object = VCalendar::new_empty();

        Ok(vcal_object)
    }
}

#[test]
fn ics_extention_verification() {
    assert_eq!(
        VCalendar::load_vcal_from_file(Path::new("test.random")).unwrap_err(),
        ICSError::NotICSFile
    );
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
