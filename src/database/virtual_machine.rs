use crate::database::command::Command;
use std::collections::VecDeque;

pub struct VirtualMachine {
    command_stack: VecDeque<Command>,
}
