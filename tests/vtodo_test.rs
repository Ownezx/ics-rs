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

#[test]
fn vtodo_read_example_2() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_2").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    let vtodo = VTodo::parse_from_bufreader(lines).unwrap();

    assert_eq!(vtodo.uid, "20070514T103211Z-123404@example.com");
    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 5, 14)
        .unwrap()
        .and_hms_opt(10, 32, 11)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 5, 14)
        .unwrap()
        .and_hms_opt(11, 0, 0)
        .unwrap();
    assert_eq!(vtodo.dtstart.unwrap(), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(2007, 7, 7)
        .unwrap()
        .and_hms_opt(10, 0, 0)
        .unwrap();
    assert_eq!(vtodo.completed.unwrap(), expected_date);

    assert_eq!(
        vtodo.summary.unwrap(),
        "Submit Revised Internet-Draft".to_string()
    );

    assert_eq!(vtodo.priority.unwrap(), 1);
    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
}

#[test]
fn vtodo_read_example_3() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_3").unwrap();
    let buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    let mut lines = buf_reader.lines();
    println!("Removing first line : {}", lines.next().unwrap().unwrap());

    let vtodo = VTodo::parse_from_bufreader(lines).unwrap();

    assert_eq!(vtodo.uid, "19970901T130000Z-123404@host.com");

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(1997, 9, 1)
        .unwrap()
        .and_hms_opt(13, 0, 0)
        .unwrap();
    assert_eq!(vtodo.dtstamp, expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(1997, 4, 15)
        .unwrap()
        .and_hms_opt(13, 30, 0)
        .unwrap();
    assert_eq!(vtodo.dtstart.unwrap(), expected_date);

    let expected_date = FixedOffset::east_opt(0)
        .unwrap()
        .ymd_opt(1997, 4, 16)
        .unwrap()
        .and_hms_opt(4, 59, 59)
        .unwrap();
    assert_eq!(vtodo.due.unwrap(), expected_date);

    assert_eq!(vtodo.summary.unwrap(), "1996 Income Tax Preparation");
    assert_eq!(vtodo.class.unwrap(), Class::CONFIDENTIAL);
    assert_eq!(vtodo.categories, vec!["FAMILY", "FINANCE"]);
    assert_eq!(vtodo.priority.unwrap(), 1);
    assert_eq!(vtodo.status.unwrap(), Status::NeedsAction);
}
