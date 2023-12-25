use crate::database::parser::parse_select;
pub struct SteelDB {}

impl SteelDB {
    pub fn new() -> SteelDB {
        return SteelDB {};
    }
    pub fn execute(&self, _command: String) {
        parse_select(_command);
    }
}
