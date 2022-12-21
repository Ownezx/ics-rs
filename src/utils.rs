use std::{
    fs::File,
    io::{BufReader, Lines},
};

#[cfg(test)]
use std::io::BufRead;

use crate::{ics_error::ICSError, properties::ParserResult};

pub fn process_multi_line_property(
    current_line: String,
    line_reader: &mut Lines<BufReader<File>>,
) -> (String, Option<Result<String, std::io::Error>>) {
    let mut out_line = current_line;

    let mut next_line = line_reader.next();
    loop {
        match next_line {
            Some(ref mut result) => match result {
                Ok(line) => {
                    if line.starts_with(' ') {
                        // Remove the first character
                        line.remove(0);
                        out_line.push_str(line);
                    } else {
                        break;
                    }
                }
                Err(_) => break,
            },
            None => break,
        }
        next_line = line_reader.next();
    }

    (out_line, next_line)
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

#[test]
fn multi_line_test() {
    let f = File::open("./tests/test_files/Other/MultiLineTest.txt").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    let first_line = lines.next().unwrap().unwrap();

    let (current_line, _) = process_multi_line_property(first_line, &mut lines);

    assert_eq!(
        current_line,
        "This is an example of a multi line string".to_string()
    );
}
