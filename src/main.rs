#![allow(unused)]
pub mod emulator;

use std::time::Instant;

use macroquad::prelude::*;

use crate::emulator::system::{
    components::registers::{Flags, Register16},
    console::Console,
};

const RESOLUTION: Vec2 = vec2(160f32, 144f32);
const SCALE_FACTOR: f32 = 4f32;
const SCALED_RESOLUTION: Vec2 = vec2(RESOLUTION.x * SCALE_FACTOR, RESOLUTION.y * SCALE_FACTOR);

fn conf() -> Conf {
    Conf {
        window_title: "Game Crab".to_string(),
        window_width: SCALED_RESOLUTION.x as i32,
        window_height: SCALED_RESOLUTION.y as i32,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut console = Console::new_with_rom("roms/Tetris.gb");
    // let mut console = Console::new();
    console.rom_into_ram();
    let mut n: u64 = 0;

    loop {
        clear_background(BLACK);
        // let start = Instant::now();
        for n in 0..70224 {
            // println!(
            //     "Tick Counter: {} PC: {:x}, RAM: {:x}",
            //     console.tick_counter,
            //     console.cpu.get_register_16(&Register16::Pc),
            //     console
            //         .ram
            //         .fetch(console.cpu.get_register_16(&Register16::Pc))
            // );
            // println!(
            //     "Z:{} N:{} H:{} C:{}",
            //     console.cpu.get_flag(&Flags::Z),
            //     console.cpu.get_flag(&Flags::N),
            //     console.cpu.get_flag(&Flags::H),
            //     console.cpu.get_flag(&Flags::C)
            // );
            console.tick();
        }
        draw_fps();
        // println!("Frame took: {:?}", start.elapsed());
        next_frame().await
    }
}

fn update_texture(texture: &Texture2D, bytes: &mut Box<[u8]>) {
    for y in 0..RESOLUTION.y as usize {
        for x in 0..RESOLUTION.x as usize {
            let start = (y * 160 * 4) + (x * 4);
            bytes[start..start + 4].copy_from_slice(&[x as u8, 0, y as u8, 255]);
        }
    }

    texture.update_from_bytes(160, 144, bytes);
}

fn render_texture(texture: &Texture2D) {
    draw_texture_ex(
        texture,
        0f32,
        0f32,
        WHITE,
        DrawTextureParams {
            dest_size: Some(SCALED_RESOLUTION),
            rotation: 0f32,
            flip_x: false,
            flip_y: false,
            pivot: None,
            source: None,
        },
    );
}
