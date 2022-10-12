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

    loop {
        music(&mut music_ready).await;

        clear_background(BLACK);

        starfield.add_star();
        starfield.update();
        starfield.draw();

        balls.draw(counter);
        counter += 0.035;

        text.draw();
        text2.draw();
        println!("fps: {}", get_fps());
        next_frame().await;
    }
}
