mod screen;
mod stars;
mod text;

use macroquad::prelude::*;
use stars::Starfield;
use std::time::Duration;
use text::Text;

#[macroquad::main("Intro")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut starfield = Starfield::new();
    let mut text = Text::new("SINUS !", 70).await;

    loop {
        clear_background(BLACK);

        starfield.add_star();
        starfield.update();
        starfield.draw();

        text.draw();
        println!("fps: {}", get_fps());
        next_frame().await;
        // std::thread::sleep(Duration::from_millis(10));
    }
}
