/*
The property is defined by the following notation:

  status     = "STATUS" statparam] ":" statvalue CRLF

  statparam  = *(";" xparam)

  statvalue  = "TENTATIVE"           ;Indicates event is
                                     ;tentative.
             / "CONFIRMED"           ;Indicates event is
                                     ;definite.
             / "CANCELLED"           ;Indicates event was
                                     ;cancelled.
     ;Status values for a "VEVENT"
  statvalue  =/ "NEEDS-ACTION"       ;Indicates to-do needs action.
             / "COMPLETED"           ;Indicates to-do completed.
             / "IN-PROCESS"          ;Indicates to-do in process of
             / "CANCELLED"           ;Indicates to-do was cancelled.
     ;Status values for "VTODO".

  statvalue  =/ "DRAFT"              ;Indicates journal is draft.
             / "FINAL"               ;Indicates journal is final.
             / "CANCELLED"           ;Indicates journal is removed.
     ;Status values for "VJOURNAL".
*/

use std::str::FromStr;

use crate::ics_error::ICSError;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    NeedsAction,
    Completed,
    InProgress,
    Tentative,
    Confirmed,
    Draft,
    Final,
    Cancelled,
}

impl Status {
    pub fn validate_vevent(&self) -> bool {
        matches!(
            self,
            Status::Tentative | Status::Confirmed | Status::Cancelled
        )
    }

    pub fn validate_vtodo(&self) -> bool {
        matches!(
            self,
            Status::NeedsAction | Status::Completed | Status::InProgress | Status::Cancelled
        )
    }

    pub fn validate_vjournal(&self) -> bool {
        matches!(self, Status::Draft | Status::Final | Status::Cancelled)
    }
}

impl std::str::FromStr for Status {
    type Err = ICSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NEEDS-ACTION" => Ok(Status::NeedsAction),
            "COMPLETED" => Ok(Status::Completed),
            "IN-PROCESS" => Ok(Status::InProgress),
            "TENTATIVE" => Ok(Status::Tentative),
            "CONFIRMED" => Ok(Status::Confirmed),
            "DRAFT" => Ok(Status::Draft),
            "FINAL" => Ok(Status::Final),
            "CANCELLED" => Ok(Status::Cancelled),
            _ => Err(ICSError::PropertyConditionNotRespected),
        }
    }
}

impl From<Status> for String {
    fn from(status: Status) -> Self {
        match status {
            Status::NeedsAction => "NEEDS-ACTION".to_string(),
            Status::Completed => "COMPLETED".to_string(),
            Status::InProgress => "IN-PROCESS".to_string(),
            Status::Tentative => "TENTATIVE".to_string(),
            Status::Confirmed => "CONFIRMED".to_string(),
            Status::Draft => "DRAFT".to_string(),
            Status::Final => "FINAL".to_string(),
            Status::Cancelled => "CANCELLED".to_string(),
        }
    }
}

#[test]
fn status_validation() {
    let status = Status::NeedsAction;
    assert!(!status.validate_vevent());
    assert!(!status.validate_vjournal());
    assert!(status.validate_vtodo());

    let status = Status::Completed;
    assert!(!status.validate_vevent());
    assert!(!status.validate_vjournal());
    assert!(status.validate_vtodo());

    let status = Status::InProgress;
    assert!(!status.validate_vevent());
    assert!(!status.validate_vjournal());
    assert!(status.validate_vtodo());

    let status = Status::Tentative;
    assert!(status.validate_vevent());
    assert!(!status.validate_vjournal());
    assert!(!status.validate_vtodo());

    let status = Status::Confirmed;
    assert!(status.validate_vevent());
    assert!(!status.validate_vjournal());
    assert!(!status.validate_vtodo());

    let status = Status::Draft;
    assert!(!status.validate_vevent());
    assert!(status.validate_vjournal());
    assert!(!status.validate_vtodo());

    let status = Status::Final;
    assert!(!status.validate_vevent());
    assert!(status.validate_vjournal());
    assert!(!status.validate_vtodo());

    let status = Status::Cancelled;
    assert!(status.validate_vevent());
    assert!(status.validate_vjournal());
    assert!(status.validate_vtodo());
}

#[test]
fn from_str() {
    assert_eq!(
        Status::from_str("NEEDS-ACTION").unwrap(),
        Status::NeedsAction
    );
    assert_eq!(Status::from_str("COMPLETED").unwrap(), Status::Completed);
    assert_eq!(Status::from_str("IN-PROCESS").unwrap(), Status::InProgress);
    assert_eq!(Status::from_str("TENTATIVE").unwrap(), Status::Tentative);
    assert_eq!(Status::from_str("CONFIRMED").unwrap(), Status::Confirmed);
    assert_eq!(Status::from_str("DRAFT").unwrap(), Status::Draft);
    assert_eq!(Status::from_str("FINAL").unwrap(), Status::Final);
    assert_eq!(Status::from_str("CANCELLED").unwrap(), Status::Cancelled);
}

#[test]
fn to_str() {
    assert_eq!(String::from(Status::NeedsAction), "NEEDS-ACTION");
    assert_eq!(String::from(Status::Completed), "COMPLETED");
    assert_eq!(String::from(Status::InProgress), "IN-PROCESS");
    assert_eq!(String::from(Status::Tentative), "TENTATIVE");
    assert_eq!(String::from(Status::Confirmed), "CONFIRMED");
    assert_eq!(String::from(Status::Draft), "DRAFT");
    assert_eq!(String::from(Status::Final), "FINAL");
    assert_eq!(String::from(Status::Cancelled), "CANCELLED");
}
