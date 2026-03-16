# Game Crab

A cycle-accurate Game Boy (DMG) emulator written in Rust. Built from scratch to faithfully reproduce the Sharp SM83 CPU, PPU, APU, and memory subsystem at the T-cycle level.

## Features

- **Full SM83 CPU** -- all standard and CB-prefixed opcodes, interrupts, HALT/STOP, and the HALT bug
- **Cycle-accurate execution** -- instructions decomposed into micro-ops, one per T-cycle (70,224 per frame)
- **Scanline PPU** -- background, window, and sprite rendering with OAM scan, BG priority, and DMG sprite sorting
- **4-channel APU** -- pulse, wave, and noise channels with envelope/sweep/length, stereo panning at 44.1 kHz via `cpal`
- **MBC1 & MBC3** cartridge mappers with ROM/RAM banking
- **Battery saves** -- automatic `.sav` persistence for battery-backed cartridges
- **Save states** -- binary state serialization (F5 save, F9 load)
- **Display** -- 160x144 framebuffer at 4x scale via `macroquad`

## Architecture

```
Console
 ├── Cpu             -- registers, flags, IME, HALT/STOP state
 ├── Ram             -- 64 KB address space, memory-mapped I/O
 │    └── Cartridge  -- ROM + MBC (NoMbc / Mbc1 / Mbc3)
 ├── Ppu             -- scanline renderer, framebuffer, sprite evaluation
 ├── Apu             -- audio channels, sample generation, ring buffer output
 └── ExecutionQueue  -- micro-op pipeline for cycle-stepped execution
```

`Console::tick()` advances the system by one T-cycle. The executor decodes opcodes into a strongly-typed `Instruction` enum, expands them into micro-ops, and drains one per tick. The PPU, timers, and interrupts are clocked in lockstep.

## Getting Started

```bash
# Build
cargo build --release

# Run
cargo run --release -- path/to/rom.gb
```

Requires the [Rust toolchain](https://rustup.rs/) (stable, 2024 edition).

## Controls

| Key | Button | | Key | Function |
|-----|--------|-|-----|----------|
| Z | A | | F5 | Save state |
| X | B | | F9 | Load state |
| Enter | Start | | | |
| Backspace | Select | | | |
| Arrows | D-Pad | | | |

## Technical Highlights

- **Micro-op execution model** -- cycle-level timing accuracy for PPU/timer interaction
- **Typed instruction decoding** -- full opcode space mapped to a Rust enum hierarchy, catching decode errors at compile time
- **Trait-based MBC abstraction** -- `Mbc` trait allows adding new mappers without modifying the memory bus
- **Lock-free audio** -- ring buffer (`ringbuf`) decouples emulation from the `cpal` audio thread
- **Bit-level hardware emulation** -- flag manipulation, bank switching, tile decoding, and duty cycle logic at the individual bit level

## Current Status

Fully playable for DMG titles using NoMBC, MBC1, or MBC3 cartridges. Not yet implemented: MBC2/MBC5, MBC3 real-time clock, Game Boy Color support, and serial link.

## License

[CC0 1.0 Universal](Licenses/LICENSE.txt)

## Disclaimer

Game Boy is a registered trademark of Nintendo Co., Ltd. This project is not affiliated with or endorsed by Nintendo. No ROMs are included.
