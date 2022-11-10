use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ical::vtodo::VTodo;

#[test]
//#[should_panic(expected = "testsdfss")]
fn VTodo_read_example_1() {
    let f = File::open("./tests/test_files/vtodo/example_vtodo_1").unwrap();
    let mut buf_reader = BufReader::new(f);

    // Consume the first VTODO line
    buf_reader.consume(1);

    let vtodo = VTodo::parse_from_bufreader(buf_reader.lines()).unwrap();

    assert_eq!(vtodo.uid, "20070313T123432Z-456553@example.com");
}
