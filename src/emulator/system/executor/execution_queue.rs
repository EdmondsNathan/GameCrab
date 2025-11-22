use crate::emulator::commands::command::Command;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub(crate) struct ExecutionQueue {
    map: HashMap<u64, VecDeque<Command>>,
}

impl ExecutionQueue {
    pub(crate) fn new() -> ExecutionQueue {
        Self::default()
    }

    pub(crate) fn push_command_absolute(&mut self, tick: u64, command: Command) {
        match self.map.get_mut(&tick) {
            Some(cmds) => {
                cmds.push_back(command);
            }
            None => {
                self.map.insert(tick, VecDeque::from([command]));
            }
        }
    }

    pub(crate) fn pop(&mut self, tick: &u64) -> Option<VecDeque<Command>> {
        self.map.remove(tick)
    }

    pub(crate) fn peek(&mut self, tick: &u64) -> Option<&VecDeque<Command>> {
        self.map.get(tick)
    }
}
