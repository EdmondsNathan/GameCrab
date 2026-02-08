use crate::emulator::commands::command::Command;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub(crate) struct ExecutionQueue {
    map: HashMap<u64, VecDeque<Command>>,
}

impl ExecutionQueue {
    /// Create a new execution queue object.
    pub(crate) fn new() -> ExecutionQueue {
        Self::default()
    }

    /// Push a command to a specific tick.
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

    /// Pop the queue at a specific tick.
    pub(crate) fn pop(&mut self, tick: &u64) -> Option<VecDeque<Command>> {
        self.map.remove(tick)
    }

    /// Peek at the queue of a specific tick.
    pub(crate) fn peek(&mut self, tick: &u64) -> Option<&VecDeque<Command>> {
        self.map.get(tick)
    }
}
