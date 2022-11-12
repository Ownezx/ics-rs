use std::{error::Error, fmt};

/// This is the list of all possible errors linked to the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ICSError {
    UnableToParseProperty,
    PropertyConditionNotRespected,
    MissingNecessaryProperty,
    DuplicateUniqueProperty,
    BeginWithoutEnd,
}

impl Error for ICSError {}

impl fmt::Display for ICSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out_str = match self {
            ICSError::DuplicateUniqueProperty => "Unique property apears twice in file",
            ICSError::BeginWithoutEnd => {
                "Reached end of file without finding the end of current object."
            }
            ICSError::MissingNecessaryProperty => "Missing necessary property.",
            ICSError::PropertyConditionNotRespected => "Propoerty condition not Respected.",
            ICSError::UnableToParseProperty => "Unable to parse property.",
        };

        write!(f, "{}", out_str)
    }
}
