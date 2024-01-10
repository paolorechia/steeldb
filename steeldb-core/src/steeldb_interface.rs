use crate::ExecutionResult;

pub trait SteelDBInterface {
    fn execute(&mut self, user_input: String) -> ExecutionResult;
}