use crate::screen::center;
use macroquad::prelude::*;

pub struct Balls {
    balls: Vec<Ball>,
}

impl Balls {
    pub async fn new(delay: f32) -> Self {
        let mut balls = Vec::new();
        let mut ball_delay = 0.0;
        for index in 0..8 {
            let ball = Ball::new(ball_delay, index);
            balls.push(ball);
            ball_delay += delay;
        }
        Self { balls }
    }

    pub fn draw(&mut self, counter: f32) {
        for ball in &mut self.balls {
            ball.draw(counter)
        }
    }
}

pub struct Ball {
    sprite: Texture2D,
    pos: Vec2,
    delay: f32,
}

impl Ball {
    fn new(delay: f32, index: usize) -> Self {
        let ferris = include_bytes!("../images/ferris.png");
        let ball = include_bytes!("../images/ball.png");
        let sprite = if index % 2 == 0 {
            Texture2D::from_file_with_format(&ferris[..], None)
        } else {
            Texture2D::from_file_with_format(&ball[..], None)
        };

        Self {
            sprite,
            pos: Vec2::ZERO,
            delay,
        }
    }

    fn draw(&mut self, counter: f32) {
        let center = center();

        self.pos = vec2(
            (center.x - self.sprite.width() / 2.0)
                + (screen_width() / 2.5) * (counter + self.delay).sin(),
            (center.y - self.sprite.height() / 2.0)
                + screen_height() / 3.0 * (1.5 * (counter + self.delay)).cos(),
        );
        draw_texture(self.sprite, self.pos.x, self.pos.y, WHITE);
    }
}
