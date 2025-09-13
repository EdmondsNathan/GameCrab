use crate::emulator::executor::*;
use crate::emulator::instruction::Instruction;
use crate::emulator::instruction::Instruction::*;
use crate::emulator::ram::RAM;
use crate::emulator::registers::Registers;
use crate::emulator::rom_loaders::rom::ROM;

pub struct CPU {
    pub(crate) registers: Registers,
    pub(crate) stack_pointer: u16,
    pub(crate) program_counter: u16,
    pub(crate) rom: ROM,
    pub(crate) ram: RAM,
    pub(crate) cb_mode: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x0100,
            rom: ROM::new(),
            ram: RAM::new(),
            cb_mode: false,
        }
    }

    pub fn new_rom(path: &str) -> CPU {
        let rom = ROM::try_new(path);
        match rom {
            Ok(rom) => {
                let mut cpu = CPU {
                    registers: Registers::new(),
                    stack_pointer: 0xFFFE,
                    program_counter: 0x0100,
                    rom,
                    ram: RAM::new(),
                    cb_mode: false,
                };
                cpu.rom_into_ram();
                cpu
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let rom = ROM::try_new(path);
        match rom {
            Ok(rom) => {
                self.rom = rom;
                self.rom_into_ram();
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    fn rom_into_ram(&mut self) {
        let mut i: u16 = 0x100;
        for byte in &self.rom.bytes {
            self.ram.set(*byte, i);
            i = i + 1;
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            CB => {
                cb_instruction();
            }
            Control(control) => {
                control_instruction(self, control);
            }
            Load16(ld16) => {
                load16(self, ld16);
            }
            Push(op) => {
                push(self, op);
            }
            Pop(op) => {
                pop(self, op);
            }
            Load8(to, from) => {
                load8(self, to, from);
            }
            Arithmetic16(op) => {
                arithmetic16(self, op);
            }
            Arithmetic8(op) => {
                arithmetic8(self, op);
            }
            JumpRelative(jr) => {
                jump_relative(self, jr);
            }
            Jump(jp) => {
                jump(self, jp);
            }
            Restart(arg) => {
                restart(self, arg);
            }
            Return(op) => {
                ret(self, op);
            }
            Call(op) => {
                call(self, op);
            }
            BitOp(op) => {
                bit_op(self, op);
            }
        }
    }
}
