use crate::emulator::{
    console::Console,
    cpu::CPU,
    registers::{Register16, Register8},
};

pub(crate) struct Command {
    source: Source,
    destination: Destination,
}

impl Command {
    pub(crate) fn new(source: Source, destination: Destination) -> Command {
        Command {
            source,
            destination,
        }
    }

    pub(crate) fn execute_command(&self, console: &mut Console) {
        let value = match &self.source {
            Source::Register(register) => console.cpu.get_register(&register),
            Source::Ram(address) => console.ram.fetch(*address),
            Source::Value(value) => *value,
        };

        match &self.destination {
            Destination::Register(register) => console.cpu.set_register(value, register),
            Destination::Ram(address) => console.ram.set(value, *address),
        }
    }
}

pub(crate) enum Source {
    Register(Register8),
    Ram(u16),
    Value(u8),
}

pub(crate) enum Destination {
    Register(Register8),
    Ram(u16),
}
