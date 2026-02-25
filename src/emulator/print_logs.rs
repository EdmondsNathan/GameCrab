use crate::emulator::system::{
    components::registers::{Flags, Register16, Register8},
    console::Console,
};

pub(crate) fn log_pc(console: &Console) -> String {
    todo!()
}

pub(crate) fn log_flags(console: &Console) -> String {
    format!(
        "Z: {}, N: {}, H: {}, C: {}",
        console.cpu.get_flag(&Flags::Z),
        console.cpu.get_flag(&Flags::N),
        console.cpu.get_flag(&Flags::H),
        console.cpu.get_flag(&Flags::C),
    )
}

pub(crate) fn log_tick_counter(console: &Console) -> String {
    format!("Tick Counter: {}", console.tick_counter)
}

pub(crate) fn log_cpu_registers(console: &mut Console) -> String {
    format!("A: {:02X}, B: {:02X}, C: {:02X}, D: {:02X}, E: {:02X}, F: {:02X}, H: {:02X}, L: {:02X}, X: {:02X}, Y: {:02X}, SP: {:04X}, PC: {:04X}, Bus: {:04X},", 
        console.cpu.get_register(&Register8::A),
        console.cpu.get_register(&Register8::B),
        console.cpu.get_register(&Register8::C),
        console.cpu.get_register(&Register8::D),
        console.cpu.get_register(&Register8::E),
        console.cpu.get_register(&Register8::F),
        console.cpu.get_register(&Register8::H),
        console.cpu.get_register(&Register8::L),
        console.cpu.get_register(&Register8::X),
        console.cpu.get_register(&Register8::Y),
        console.cpu.get_register_16(&Register16::Sp),
        console.cpu.get_register_16(&Register16::Pc),
        console.cpu.get_register_16(&Register16::Bus),
        )
}

pub(crate) fn log_ram_address(console: &Console, address: u16) -> String {
    format!(
        "Address: {:04X}, RAM: {:02X}",
        address,
        console.ram.fetch(address)
    )
}

pub(crate) fn log_dump_ram_nonzero(console: &Console) -> String {
    let mut ram = "".to_owned();
    for n in 0..=65535 {
        if console.ram.fetch(n) == 0 {
            continue;
        }
        let append = format!("{:04X}: {:02X}\n", n, console.ram.fetch(n));
        ram = ram + &append;
    }

    ram
}

pub(crate) fn log_dump_ram(console: &Console) -> String {
    let mut ram = "".to_owned();
    for n in 0..=65535 {
        let append = format!("{:04X}: {:02X}\n", n, console.ram.fetch(n));
        ram = ram + &append;
    }

    ram
}
