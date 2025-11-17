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

        cpu.set_register(1, &Register8::A);
        cpu.cpu_add(15, &Register8::A, true);
        assert_eq!(16, cpu.get_register(&Register8::A));
        assert!(!cpu.get_flag(&Flags::Z));
        assert!(!cpu.get_flag(&Flags::N));
        assert!(cpu.get_flag(&Flags::H));
        assert!(!cpu.get_flag(&Flags::C));
    }
}
