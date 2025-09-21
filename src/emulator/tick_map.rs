use crate::emulator::console::Console;
use std::collections::{HashMap, VecDeque};

pub(crate) struct TickMap {
    map: HashMap<u64, VecDeque<fn(&mut Console)>>,
}

impl TickMap {
    pub(crate) fn new() -> TickMap {
        TickMap {
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

    pub(crate) fn remove(&mut self, tick: &u64) -> Option<VecDeque<fn(&mut Console)>> {
        self.map.remove(&tick)
    }
}
