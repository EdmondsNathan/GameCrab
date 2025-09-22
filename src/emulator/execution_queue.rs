use crate::emulator::console::Console;
use std::collections::{HashMap, VecDeque};

pub(crate) struct ExecutionQueue {
    map: HashMap<u64, VecDeque<fn(&mut Console)>>,
}

impl ExecutionQueue {
    pub(crate) fn new() -> ExecutionQueue {
        ExecutionQueue {
            map: HashMap::new(),
        }
    }

    pub(crate) fn push_command(&mut self, tick: u64, command: fn(&mut Console)) {
        match self.map.get_mut(&tick) {
            Some(cmds) => {
                cmds.push_back(command);
            }
            None => {
                self.map.insert(tick, VecDeque::from([command]));
            }
        }
    }

    pub(crate) fn merge(&mut self, execution_queue: ExecutionQueue) {
        for queue in execution_queue.map {
            for command in queue.1 {
                self.push_command(queue.0, command);
            }
        }
    }

    pub(crate) fn pop(&mut self, tick: &u64) -> Option<VecDeque<fn(&mut Console)>> {
        self.map.remove(tick)
    }
}
