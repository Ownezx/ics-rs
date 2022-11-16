/*
The property is defined by the following notation:

  class      = "CLASS" classparam ":" classvalue CRLF

  classparam = *(";" xparam)
  classvalue = "PUBLIC" / "PRIVATE" / "CONFIDENTIAL" / iana-token
             / x-name
  ;Default is PUBLIC
*/

use crate::ics_error::ICSError;
use std::str::FromStr;

/// This property defines the access classification for a calendar component.
#[derive(Debug, PartialEq, Eq)]
pub enum Class {
    PUBLIC,
    PRIVATE,
    CONFIDENTIAL,
    IANATOKEN(String),
    XNAME(String),
}

impl std::str::FromStr for Class {
    type Err = ICSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PUBLIC" => Ok(Class::PUBLIC),
            "PRIVATE" => Ok(Class::PRIVATE),
            "CONFIDENTIAL" => Ok(Class::CONFIDENTIAL),
            _ => todo!("Not implemented for IANA-TOKEN or X-NAME"),
        }
    }
}

impl From<Class> for String {
    fn from(class: Class) -> Self {
        match class {
            Class::PUBLIC => "PUBLIC".to_string(),
            Class::PRIVATE => "PRIVATE".to_string(),
            Class::CONFIDENTIAL => "CONFIDENTIAL".to_string(),
            Class::IANATOKEN(string) => string,
            Class::XNAME(string) => string,
        }
    }
}

#[test]
fn from_str() {
    assert_eq!(Class::from_str("PUBLIC").unwrap(), Class::PUBLIC);
    assert_eq!(Class::from_str("PRIVATE").unwrap(), Class::PRIVATE);
    assert_eq!(
        Class::from_str("CONFIDENTIAL").unwrap(),
        Class::CONFIDENTIAL
    );
    // Need to add X-NAME and IANA-TOKEN
}

#[test]
fn to_str() {
    assert_eq!(String::from(Class::PUBLIC), "PUBLIC");
    assert_eq!(String::from(Class::PRIVATE), "PRIVATE");
    assert_eq!(String::from(Class::CONFIDENTIAL), "CONFIDENTIAL");
    // Need to add X-NAME and IANA-TOKEN
}
