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
}
