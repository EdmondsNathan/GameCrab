use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
};

pub(crate) enum Command {
    Read(Source, Destination),
    Update(fn(&mut Console)),
}

impl Command {
    /// Execute a read or update command.
    pub(crate) fn execute_command(&self, console: &mut Console) {
        match self {
            Command::Read(source, destination) => Self::read(console, source, destination),
            Command::Update(func) => func(console),
        }
    }

    /// Execute a read command.
    fn read(console: &mut Console, source: &Source, destination: &Destination) {
        let value = match source {
            Source::Register(register) => console.cpu.get_register(register),
            Source::RamFromRegister(register16) => {
                console.ram.fetch(console.cpu.get_register_16(register16))
            }
        };

        match destination {
            Destination::Register(register) => console.cpu.set_register(value, register),
            Destination::RamFromRegister(register16) => console
                .ram
                .set(value, console.cpu.get_register_16(register16)),
        }
    }
}

pub(crate) enum Source {
    Register(Register8),
    RamFromRegister(Register16),
}

pub(crate) enum Destination {
    Register(Register8),
    RamFromRegister(Register16),
}
