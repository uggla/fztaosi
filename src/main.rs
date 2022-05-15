mod screen;
mod stars;
mod text;

use macroquad::prelude::*;
use stars::Starfield;
use text::Text;

#[macroquad::main("Intro")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut starfield = Starfield::new();
    let mut text = Text::new("THIS IS AN OLD SCHOOL SINUS SCROLL !!!    ENJOY IT...    GREETINGS TO ROSE - TAUPIE - MENTHE...", 70).await;
    let mut text2 = Text::new("HORIZONTAL SCROLLING...", 70).await;

    loop {
        let delta = get_frame_time();
        clear_background(BLACK);

        starfield.add_star();
        starfield.update();
        starfield.draw();

        text.draw(delta, 400.0, 0.0, true, RED, true);
        text2.draw(
            delta,
            200.0,
            screen_height() / 2.0 - 10.0,
            false,
            RED,
            false,
        );
        println!("fps: {}", get_fps());
        next_frame().await;
    }
}
