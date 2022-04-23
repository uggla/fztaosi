mod screen;
use macroquad::prelude::*;
use screen::center;

#[macroquad::main("Intro")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let center = center();
    loop {
        clear_background(BLACK);
        draw_circle(center.x, center.y, 3.0, WHITE);

        next_frame().await
    }
}
