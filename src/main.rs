use macroquad::prelude::*;

#[macroquad::main("Intro")]
async fn main() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    rand::srand(miniquad::date::now() as u64);

    loop {
        clear_background(BLACK);
        draw_circle(center_x, center_y, 3.0, WHITE);

        next_frame().await
    }
}
