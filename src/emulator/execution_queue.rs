use crate::emulator::{
    console::Console,
    cpu::CPU,
    registers::{Register16, Register8},
};
use std::collections::{HashMap, VecDeque};

pub(crate) struct ExecutionQueue {
    map: HashMap<u64, VecDeque<Command>>,
}

pub(crate) enum Command {
    Standard(fn(&mut Console)),
    ReadWrite(fn(&mut Console, u16, Register8), u16, Register8),
    SetRegister(fn(&mut CPU, u8, Register8), u8, Register8),
    SetRegister16(fn(&mut CPU, u16, Register16), u16, Register16),
}

impl ExecutionQueue {
    pub(crate) fn new() -> ExecutionQueue {
        ExecutionQueue {
            map: HashMap::new(),
        }
    }

    pub(crate) fn push_command(&mut self, tick: u64, command: Command) {
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
}
