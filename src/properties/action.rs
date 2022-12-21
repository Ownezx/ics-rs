/*
 Format Definition:  This property is defined by the following
      notation:

       action      = "ACTION" actionparam ":" actionvalue CRLF

       actionparam = *(";" other-param)


       actionvalue = "AUDIO" / "DISPLAY" / "EMAIL"
                   / iana-token / x-name

*/

use crate::ics_error::ICSError;

#[cfg(test)]
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Audio,
    Display,
    Email,
}

impl std::str::FromStr for Action {
    type Err = ICSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AUDIO" => Ok(Action::Audio),
            "DISPLAY" => Ok(Action::Display),
            "EMAIL" => Ok(Action::Email),
            _ => Err(ICSError::PropertyConditionNotRespected(
                "ACTION".to_string(),
            )),
        }
    }
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        match action {
            Action::Audio => "AUDIO".to_string(),
            Action::Display => "DISPLAY".to_string(),
            Action::Email => "EMAIL".to_string(),
        }
    }
}

#[test]
fn from_str() {
    assert_eq!(Action::from_str("AUDIO").unwrap(), Action::Audio);
    assert_eq!(Action::from_str("DISPLAY").unwrap(), Action::Display);
    assert_eq!(Action::from_str("EMAIL").unwrap(), Action::Email);
}

#[test]
fn to_str() {
    assert_eq!(String::from(Action::Audio), "AUDIO");
    assert_eq!(String::from(Action::Display), "DISPLAY");
    assert_eq!(String::from(Action::Email), "EMAIL");
}
