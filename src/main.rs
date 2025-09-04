use macroquad::prelude::*;

const RESOLUTION: Vec2 = vec2(160f32, 144f32);
const SCALE_FACTOR: f32 = 4f32;
const SCALED_RESOLUTION: Vec2 = vec2(RESOLUTION.x * SCALE_FACTOR, RESOLUTION.y * SCALE_FACTOR);

fn conf() -> Conf {
    Conf {
        window_title: "Game Crab".to_string(),
        window_width: SCALED_RESOLUTION.x as i32,
        window_height: SCALED_RESOLUTION.y as i32 ,
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
    loop {
        clear_background(BLACK);

        draw_fps();

        next_frame().await
    }
}
