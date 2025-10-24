use crate::emulator::{console::Console, registers::Register8};

pub(crate) enum Command {
    Read(Source, Destination),
    Update(fn(&mut Console)),
}

impl Command {
    pub(crate) fn execute_command(&self, console: &mut Console) {
        match self {
            Command::Read(source, destination) => Self::read(console, source, destination),
            Command::Update(func) => func(console),
        }
    }

    fn read(console: &mut Console, source: &Source, destination: &Destination) {
        let value = match source {
            Source::Register(register) => console.cpu.get_register(register),
            Source::Ram(address) => console.ram.fetch(*address),
            Source::Value(value) => *value,
        };

        match destination {
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
