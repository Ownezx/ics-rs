extern crate ical;

#[test]
#[should_panic(expected = "BLAHBLAH")]
fn read_string_property() {
    let test_string: String = "PRODID:-//ics-rs//".to_string();
}
