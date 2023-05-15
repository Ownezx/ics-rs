use crate::ics_error::ICSError;

#[derive(Debug)]
pub struct VJournal {}
impl VJournal {
    pub fn parse_from_bufreader(
        _line_reader: &mut std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Result<VJournal, ICSError> {
        todo!()
    }
}
