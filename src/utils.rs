use std::{
    fs::File,
    io::{BufReader, Lines},
};

use crate::{ics_error::ICSError, properties::ParserResult};

pub fn process_multi_line_property(
    current_line: String,
    line_reader: &mut Lines<BufReader<File>>,
) -> (String, Option<Result<String, std::io::Error>>) {
    eprintln!("Placeholder present but not implemented for process_multi_line_property");
    (current_line, line_reader.next())
}

pub fn apply_unique_property<T: std::convert::From<crate::properties::ParserResult>>(
    arg: &mut Option<T>,
    value: ParserResult,
    property_name: String,
) -> Result<(), ICSError> {
    match arg {
        Some(_) => Err(ICSError::DuplicateUniqueProperty(property_name)),
        None => {
            *arg = Some(T::try_from(value).unwrap());
            Ok(())
        }
    }
}
