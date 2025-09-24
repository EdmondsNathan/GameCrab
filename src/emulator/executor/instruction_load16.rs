use crate::emulator::{
    console::Console,
    cpu::CPU,
    execution_queue::Command,
    instruction::Ld16,
    registers::{Register16, Register8},
};

impl Console {
    pub(super) fn instruction_load16(&mut self, ld16: Ld16) -> Option<u64> {
        match ld16 {
            Ld16::BCU16 => {
                pc_increments(self);
                load_registers(self, Register8::B, Register8::C);
                return Some(12);
            }
            Ld16::DEU16 => {
                pc_increments(self);
                load_registers(self, Register8::D, Register8::E);
                return Some(12);
            }
            Ld16::HLU16 => {
                pc_increments(self);
                load_registers(self, Register8::H, Register8::L);
                return Some(12);
            }
            Ld16::SPU16 => {
                pc_increments(self);
                load_registers(self, Register8::SpHigh, Register8::SpLow);
                return Some(12);
            }
            Ld16::U16SP => {
                pc_increments(self);
                self.execution_queue.push_command(
                    self.tick_counter + 12,
                    Command::ReadWrite(
                        Console::command_register_to_ram,
                        self.cpu.get_register_16(Register16::Pc) + 12,
                        Register8::SpLow,
                    ),
                );
                self.execution_queue.push_command(
                    self.tick_counter + 16,
                    Command::ReadWrite(
                        Console::command_register_to_ram,
                        self.cpu.get_register_16(Register16::Pc) + 16,
                        Register8::SpHigh,
                    ),
                );
                return Some(20);
            }
            Ld16::SPHL => {
                self.execution_queue.push_command(
                    self.tick_counter + 3,
                    Command::SetRegister(
                        CPU::set_register,
                        self.cpu.get_register(Register8::H),
                        Register8::SpHigh,
                    ),
                );
                self.execution_queue.push_command(
                    self.tick_counter + 3,
                    Command::SetRegister(
                        CPU::set_register,
                        self.cpu.get_register(Register8::L),
                        Register8::SpLow,
                    ),
                );
                return Some(8);
            }
        }

        fn load_registers(
            console: &mut Console,
            register_high: Register8,
            register_low: Register8,
        ) {
            console.execution_queue.push_command(
                console.tick_counter + 5,
                Command::ReadWrite(
                    Console::command_ram_to_register,
                    console.cpu.get_register_16(Register16::Pc) + 5,
                    register_low,
                ),
            );

            console.execution_queue.push_command(
                console.tick_counter + 8,
                Command::ReadWrite(
                    Console::command_ram_to_register,
                    console.cpu.get_register_16(Register16::Pc) + 8,
                    register_high,
                ),
            );
        }

        fn pc_increments(console: &mut Console) {
            console.execution_queue.push_command(
                console.tick_counter + 4,
                Command::Standard(Console::command_increment_pc),
            );
            console.execution_queue.push_command(
                console.tick_counter + 7,
                Command::Standard(Console::command_increment_pc),
            );
        }
    }
}
