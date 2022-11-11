use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ics::vtodo::VTodo;

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
}
