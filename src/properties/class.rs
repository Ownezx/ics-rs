/*
The property is defined by the following notation:

  class      = "CLASS" classparam ":" classvalue CRLF

  classparam = *(";" xparam)
  classvalue = "PUBLIC" / "PRIVATE" / "CONFIDENTIAL" / iana-token
             / x-name
  ;Default is PUBLIC
*/

/// This property defines the access classification for a calendar component.
pub enum Class {
    PUBLIC,
    PRIVATE,
    CONFIDENTIAL,
    IANA_TOKEN,
    X_NAME,
}
