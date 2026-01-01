use crate::emulator::system::components::{cpu::Cpu, registers::Flags, registers::Register8};

impl Cpu {
    /// Set register = register + value.
    ///
    /// Flags decides if this operation modifies the flags register.
    pub(crate) fn cpu_add(&mut self, value: u8, register: &Register8, flags: bool) {
        let original = self.get_register(register);
        let (new, overflow) = original.overflowing_add(value);
        self.set_register(new, register);

        if !flags {
            return;
        }

        self.set_flag(self.get_register(register) == 0, &Flags::Z);
        self.set_flag(false, &Flags::N);
        self.set_flag((original & 0xF) + (value & 0xF) > 0xF, &Flags::H);
        self.set_flag(overflow, &Flags::C);
    }

    /// Set register = register + (value + carry).
    ///
    /// Flags decides if this operation modifies the flags register.
    pub(crate) fn cpu_adc(&mut self, value: u8, register: &Register8, flags: bool) {
        let original = self.get_register(register);

        let carry = match self.get_flag(&Flags::C) {
            true => 1,
            false => 0,
        };

        // Check if there is an overflow when:
        //      adding original + value, OR:
        //      adding the result of the previous expression + carry_from_bit
        let (original_plus_value, carry_from_value) = original.overflowing_add(value);
        let (result, carry_from_bit) = original_plus_value.overflowing_add(carry);
        let did_carry = carry_from_value || carry_from_bit;

        self.set_register(result, register);

        if !flags {
            return;
        }

        self.set_flag(self.get_register(register) == 0, &Flags::Z);

        self.set_flag(false, &Flags::N);

        /* The half carry checks if the first nibble of each byte
         * overflows into the second nibble.
         * EXAMPLE:
         *   0b00001111
         *  +0b00000001
         *  ___________
         *   0b00010000
         *        ^
         *  This bit is carried from the first nibble into the second.
         */
        self.set_flag(
            (original & 0xF) + (value & 0xF) + (carry & 0xF) > 0xF,
            &Flags::H,
        );

        self.set_flag(did_carry, &Flags::C);
    }

    /// Set register = register - value.
    ///
    /// Flags decides if this operation modifies the flags register.
    pub(crate) fn cpu_sub(&mut self, value: u8, register: &Register8, flags: bool) {
        let original = self.get_register(register);
        let (new, overflow) = original.overflowing_sub(value);
        self.set_register(new, register);

        if !flags {
            return;
        }

        self.set_flag(self.get_register(register) == 0, &Flags::Z);
        self.set_flag(true, &Flags::N);
        self.set_flag((original & 0xF) < (value & 0xF), &Flags::H);
        self.set_flag(overflow, &Flags::C);
    }

    /// Set register = register - (value + carry).
    ///
    /// Flags decides if this operation modifies the flags register.
    pub(crate) fn cpu_sbc(&mut self, value: u8, register: &Register8, flags: bool) {
        let original = self.get_register(register);
        let carry = match self.get_flag(&Flags::C) {
            true => 1,
            false => 0,
        };

        // result = original - (value - carry)
        //                   + (-value + carry)
        // subtract =        - (value + carry)
        let subtract = value.wrapping_add(carry);
        // result = original - subtract
        let result = original.wrapping_sub(subtract);
        // did_borrow = original < subtract
        let did_borrow = original < subtract;

        self.set_register(result, register);

        if (!flags) {
            return;
        }

        self.set_flag(self.get_register(register) == 0, &Flags::Z);

        self.set_flag(true, &Flags::N);

        //H flag
        // let (half_carry_value, mut half_carry) = (original & 0xF).overflowing_sub(value & 0xF);
        // if !half_carry {
        //     let (_, half_carry) = (half_carry_value & 0xF).overflowing_sub((carry) & 0xF);
        // }
        let did_half_carry = (original & 0xF) < ((value & 0xF) + (carry & 0xF));
        self.set_flag(did_half_carry, &Flags::H);

        self.set_flag(did_borrow, &Flags::C);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::components::{
        cpu::Cpu,
        registers::{Flags, Register8},
    };

    #[test]
    fn add() {
        let mut cpu = Cpu::new();
        cpu.set_register(254, &Register8::A);

        cpu.cpu_add(1, &Register8::A, true);
        assert_eq!(255, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(!cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));

        cpu.cpu_add(1, &Register8::A, true);
        assert_eq!(0, cpu.get_register(&Register8::A));
        assert!(cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(cpu.get_flag(&Flags::C));

        cpu.set_register(0b00000001, &Register8::A);
        cpu.cpu_add(0b00001111, &Register8::A, true);
        assert_eq!(0b00010000, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));
    }

    #[test]
    fn adc() {
        let mut cpu = Cpu::new();
        cpu.set_register(253, &Register8::A);

        cpu.set_flag(true, &Flags::C);
        cpu.cpu_adc(1, &Register8::A, true);
        assert_eq!(255, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(!cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));

        cpu.set_flag(true, &Flags::C);
        cpu.cpu_adc(1, &Register8::A, true);
        assert_eq!(1, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(cpu.get_flag(&Flags::C));

        cpu.set_register(0, &Register8::A);
        cpu.set_flag(true, &Flags::C);
        cpu.cpu_adc(0b00001111, &Register8::A, true);
        assert_eq!(0b00010000, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));
    }

    #[test]
    fn sub() {
        let mut cpu = Cpu::new();
        cpu.set_register(1, &Register8::A);

        cpu.cpu_sub(1, &Register8::A, true);
        assert_eq!(0, cpu.get_register(&Register8::A));
        assert!(cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(!cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));

        cpu.cpu_sub(1, &Register8::A, true);
        assert_eq!(255, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(cpu.get_flag(&Flags::C));

        cpu.set_register(0b00010000, &Register8::A);
        cpu.cpu_sub(0b00001000, &Register8::A, true);
        assert_eq!(0b00001000, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));
    }

    #[test]
    fn sbc() {
        let mut cpu = Cpu::new();
        cpu.set_register(2, &Register8::A);

        cpu.set_flag(true, &Flags::C);
        cpu.cpu_sbc(1, &Register8::A, true);
        assert_eq!(0, cpu.get_register(&Register8::A));
        assert!(cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(!cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));

        cpu.set_flag(true, &Flags::C);
        cpu.cpu_sbc(0, &Register8::A, true);
        assert_eq!(0b11111111, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(cpu.get_flag(&Flags::C));

        cpu.set_flag(false, &Flags::C);
        cpu.set_register(0b00010000, &Register8::A);
        cpu.cpu_sbc(0b00001000, &Register8::A, true);
        assert_eq!(0b00001000, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));

        cpu.set_flag(true, &Flags::C);
        cpu.set_register(0b00010000, &Register8::A);
        cpu.cpu_sbc(0, &Register8::A, true);
        assert_eq!(0b00001111, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));
    }
}
