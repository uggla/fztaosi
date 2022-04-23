mod screen;
mod stars;
use macroquad::prelude::*;
use stars::Starfield;

#[macroquad::main("Intro")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut starfield = Starfield::new();

    loop {
        clear_background(BLACK);

        starfield.add_star();
        starfield.update();
        starfield.draw();

        next_frame().await
    }
}
