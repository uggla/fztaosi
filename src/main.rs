use macroquad::prelude::*;

#[macroquad::main("Intro")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
