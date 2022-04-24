use crate::screen::center;
use macroquad::prelude::*;

pub struct Text {
    text: String,
    font_size: u16,
    font: Font,
    scroll_posx: f32,
}

impl Text {
    pub async fn new(msg: &str, size: u16) -> Self {
        Self {
            text: msg.to_string(),
            font_size: size,
            font: load_ttf_font("fonts/origami.ttf").await.unwrap(),
            scroll_posx: screen_width() / 2.0,
        }
    }

    fn split_text(&self) -> Vec<String> {
        let mut letters: Vec<String> = Vec::new();
        for letter in self.text.chars() {
            letters.push(letter.to_string());
        }
        letters
    }

    pub fn draw(&mut self) {
        let letters = self.split_text();
        let mut letter_posx = self.scroll_posx;
        for letter in letters {
            let dimtext = measure_text(&letter, Option::Some(self.font), self.font_size, 1.0);
            self.draw_letter(letter, Vec2::new(letter_posx, screen_height() / 2.0 - 10.0));
            letter_posx += dimtext.width;
        }
        self.scroll_posx -= 10.0;
        if letter_posx < -screen_width() / 2.0 {
            self.scroll_posx = screen_width() / 2.0;
        }
    }

    fn draw_letter(&self, letter: String, pos: Vec2) {
        let center = center();
        let pos = center + pos;
        draw_text_ex(
            &letter,
            pos.x,
            pos.y,
            TextParams {
                font_size: self.font_size,
                font: self.font,
                color: RED,
                ..Default::default()
            },
        );
    }
}
