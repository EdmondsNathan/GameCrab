#![allow(unused)]
pub mod emulator;

use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use macroquad::prelude::*;
use ringbuf::traits::{Consumer, Split};

use crate::emulator::{
    print_logs::{log_dump_ram, log_dump_ram_nonzero},
    system::{
        components::registers::{Flags, Register16},
        console::Console,
    },
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
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_path = match args.get(1) {
        Some(path) => path.as_str(),
        None => {
            eprintln!("Usage: gamecrab <rom_path>");
            eprintln!("No ROM path provided.");
            return;
        }
    };
    let mut console = Console::new_with_rom(rom_path);

    // Set up audio output with cpal
    let _audio_stream = setup_audio(&mut console);

    let texture = Texture2D::from_rgba8(160, 144, &[0; 160 * 144 * 4]);
    texture.set_filter(FilterMode::Nearest);

    // print!("{}", log_dump_ram_nonzero(&console));
    // return;

    let mut frame_counter: u32 = 0;

    loop {
        if is_quit_requested() {
            console.save_ram();
            break;
        }
        prevent_quit();

        clear_background(BLACK);

        poll_joypad(&mut console);

        if is_key_pressed(KeyCode::F5) {
            console.save_state();
        }
        if is_key_pressed(KeyCode::F9) {
            console.load_state();
        }

        for n in 0..70224 {
            console.tick();
        }

        frame_counter += 1;
        if frame_counter % 60 == 0 {
            console.save_ram();
        }

        framebuffer_to_texture(&texture, console.get_framebuffer());
        render_texture(&texture);
        // draw_fps();
        next_frame().await
    }
}

fn setup_audio(console: &mut Console) -> Option<cpal::Stream> {
    let host = cpal::default_host();
    let device = match host.default_output_device() {
        Some(d) => d,
        None => {
            eprintln!("No audio output device found");
            return None;
        }
    };

    let config = cpal::StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Default,
    };

    // Large buffer to absorb timing jitter between emulation and audio threads
    let ring = ringbuf::HeapRb::<f32>::new(16384);
    let (producer, mut consumer) = ring.split();
    console.apu.set_producer(producer);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // On underrun, hold the last sample value to avoid pops
                let mut last = 0.0f32;
                for sample in data.iter_mut() {
                    if let Some(s) = consumer.try_pop() {
                        last = s;
                    }
                    *sample = last;
                }
            },
            |err| {
                eprintln!("Audio stream error: {}", err);
            },
            None,
        )
        .ok()?;

    stream.play().ok()?;
    Some(stream)
}

fn poll_joypad(console: &mut Console) {
    // Action buttons (active low: 0 = pressed, 1 = released)
    let action = (!is_key_down(KeyCode::Z) as u8)           // A
        | ((!is_key_down(KeyCode::X) as u8) << 1)           // B
        | ((!is_key_down(KeyCode::Backspace) as u8) << 2)   // Select
        | ((!is_key_down(KeyCode::Enter) as u8) << 3); // Start

    // Direction buttons (active low)
    let direction = (!is_key_down(KeyCode::Right) as u8)
        | ((!is_key_down(KeyCode::Left) as u8) << 1)
        | ((!is_key_down(KeyCode::Up) as u8) << 2)
        | ((!is_key_down(KeyCode::Down) as u8) << 3);

    console.update_joypad(action, direction);
}

fn framebuffer_to_texture(texture: &Texture2D, framebuffer: [u8; 160 * 144 * 4]) {
    texture.update_from_bytes(160, 144, &framebuffer);
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
