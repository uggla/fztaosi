mod screen;
use macroquad::prelude::*;
use screen::center;

const MAX_SIZE: f32 = 8.0;

struct Star {
    pos: Vec2,
    size: f32,
    vel: Vec2,
    color: Color,
}

impl Star {
    fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            size: 3.0,
            vel: gen_vel(),
            color: BLACK,
        }
    }

    fn draw(&self) {
        let center = center();
        let pos = center + self.pos;
        draw_circle(pos.x, pos.y, self.size, self.color)
    }

    fn update_size(&mut self) {
        self.size = self.pos.y.abs() * MAX_SIZE / (screen_height() / 2.0);
    }

    fn update_color(&mut self) {
        let color = self.pos.y.abs() * 1.0 / (screen_height() / 2.0);
        self.color = Color {
            r: color,
            g: color,
            b: color,
            a: 1.0,
        };
    }

    fn update_pos(&mut self) {
        self.pos += self.vel;
        self.update_size();
        self.update_color();
        if self.pos.x.abs() > screen_width() / 2.0 || self.pos.y.abs() > screen_height() / 2.0 {
            self.pos = Vec2::ZERO;
            self.vel = gen_vel();
        }
    }
}

fn gen_vel() -> Vec2 {
    Vec2::new(rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0))
}

#[macroquad::main("Intro")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut star = Star::new();

    loop {
        clear_background(BLACK);
        // draw_circle(center.x, center.y, 3.0, WHITE);
        star.update_pos();
        star.draw();

        next_frame().await
    }
}
