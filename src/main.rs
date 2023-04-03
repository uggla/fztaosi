mod balls;
mod music;
mod screen;
mod stars;
mod text;

use balls::Balls;
use macroquad::prelude::*;
use music::music;
use stars::Starfield;
use text::Text;

fn display_fps(fps: &mut i32, frame_t: f32, fps_refresh: &mut f32) {
    if frame_t - *fps_refresh > 0.2 {
        *fps = get_fps();
        *fps_refresh = frame_t;
    }
    let text = format!("{} fps", fps);
    let font_size = 30.;
    draw_text(&text, 5., 20., font_size, DARKGRAY)
}

#[macroquad::main("From zero to an old school intro")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut starfield = Starfield::new();

    let mut text = Text::new(
        "THIS IS AN OLD SCHOOL SINUS SCROLL DONE WITH RUST AND MACROQUAD !!!    \
        ENJOY IT...    LAB: FROM ZERO TO AN OLD SCHOLL INTRO    \
        GREETINGS TO ROSE - TAUPIE - MENTHE...",
        70,
    )
    .speed(8.0)
    .sinus(true)
    .color(RED)
    .rainbow(true)
    .build();

    let mut text2 = Text::new("THANKS TO BE HERE WITH US AT FLOSSCON 2022...", 75)
        .speed(4.0)
        .offset_y(screen_height() / 2.0 - 10.0)
        .color(DARKBLUE)
        .build();

    let mut balls = Balls::new(0.2).await;
    let mut counter = 0.0;
    let mut music_ready = false;

    let mut show_fps: bool = false;
    let mut time = 0.0;
    let mut fps = 0;
    let mut fps_t = 0.0;
    let mut debounce_t = 0.0;
    const FPS: f32 = 1.0 / 60.0;

    loop {
        let frame_t = get_frame_time();
        let ratio = FPS / frame_t;

        time += frame_t;
        if is_key_down(KeyCode::Escape) {
            break;
        };

        if is_key_down(KeyCode::F) && time - debounce_t > 0.2 {
            if show_fps {
                show_fps = false;
            } else {
                show_fps = true;
            }
            debounce_t = time;
        };

        music(&mut music_ready).await;

        clear_background(BLACK);

        starfield.add_star();
        starfield.update();
        starfield.draw();

        balls.draw(counter);
        counter += 0.035;

        text.draw(ratio);
        text2.draw(ratio);
        if show_fps {
            display_fps(&mut fps, time, &mut fps_t);
        }
        // println!("fps: {}", get_fps());
        macroquad_profiler::profiler(Default::default());
        next_frame().await;
    }
}
