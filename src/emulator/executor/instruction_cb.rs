use crate::emulator::{console::Console, decoder::decode_cb, registers::Register16};

impl Console {
    pub(super) fn instruction_cb(&mut self) {
        let instruction = match decode_cb(self.ram.fetch(self.cpu.get_register_16(Register16::Pc)))
        {
            Ok(value) => value,
            Err(error) => panic!("{error}"),
        };

        self.execute(instruction);
    }
}
