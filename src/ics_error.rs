use std::{error::Error, fmt};

/// This is the list of all possible errors linked to the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ICSError {
    /// A necessary property is missing from a component
    MissingNecessaryProperty(String),
    /// Component has a duplicate property that should be unique
    DuplicateUniqueProperty(String),
    /// Component or vcalendar does not have end
    BeginWithoutEnd,
    /// Does not have BEGIN:VCALENDAR
    NoBegin,
    /// Cannot parse the property outlined
    UnableToParseProperty(String),
    /// The property is not recognised in the property list
    UknownProperty(String),
    /// The property is not expected in this component
    UnexpectedProperty(String),
    /// The parsed property is invalid given it's constraints
    PropertyConditionNotRespected(String),
    /// Was not able to parse the begin line of a component
    InvalidBeginLine(String),
    /// The component is not recognised
    UnknownComponent(String),
    /// The component is not expected in the parent component
    UnexpectedComponent(String),
    /// Trying to open a file without ics extension
    NotICSFile,
    /// The file reader has failed reading the file
    ReadError,
}

impl Error for ICSError {}

impl fmt::Display for ICSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
