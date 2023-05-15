use crate::ics_error::ICSError;

#[derive(Debug)]
pub struct VEvent {}
impl VEvent {
    pub fn parse_from_bufreader(
        _line_reader: &mut std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Result<VEvent, ICSError> {
        todo!()
    }
}
