const MAX_SIZE: f32 = 8.0;
const MAX_STARS: usize = 200;

use crate::screen::center;
use macroquad::prelude::*;

pub struct Starfield {
    count: usize,
    stars: Vec<Star>,
}

impl Starfield {
    pub fn new() -> Self {
        Self {
            count: 0,
            stars: Vec::new(),
        }
    }

    /// Adds a star to the sky.
    pub fn add_star(&mut self) {
        if self.count < MAX_STARS {
            self.stars.push(Star::new());
            self.count += 1;
        }
    }

    pub fn update(&mut self) {
        for star in &mut self.stars {
            star.update_pos();
        }
    }

    pub fn draw(&self) {
        for star in &self.stars {
            star.draw();
        }
    }
}

struct Star {
    pos: Vec2,
    size: f32,
    vel: Vec2,
    color: Color,
}

fn gen_vel() -> Vec2 {
    Vec2::new(rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0))
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
            self.color = BLACK;
        }
    }
}
