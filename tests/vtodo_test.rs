use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::{FixedOffset, TimeZone};
use ics::{
    properties::{class::Class, status::Status},
    vtodo::VTodo,
};

#[test]
//#[should_panic(expected = "testsdfss")]
fn vtodo_read_example_1() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_1").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    let vtodo = VTodo::parse_from_bufreader(lines).unwrap();

    assert_eq!(vtodo.uid, "20070313T123432Z-456553@example.com");
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 3, 13)
        .unwrap()
        .and_hms_opt(12, 34, 32)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 5, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    assert_eq!(vtodo.due.unwrap(), expected_date);
    assert_eq!(
        vtodo.summary.unwrap(),
        "Submit Quebec Income Tax Return for 2006".to_string()
    );
    assert_eq!(vtodo.class.unwrap(), Class::CONFIDENTIAL);
    assert_eq!(vtodo.categories, vec!["FAMILY", "FINANCE"]);
    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
}
