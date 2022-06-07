use crate::screen::center;
use macroquad::prelude::*;

pub struct Text {
    text: String,
    font_size: u16,
    font: Font,
    scroll_posx: f32,
}

impl Text {
    pub fn new(msg: &str, size: u16) -> Self {
        let font = include_bytes!("../fonts/origami.ttf");
        let font = load_ttf_font_from_bytes(font).unwrap();

        Self {
            text: msg.to_string(),
            font_size: size,
            font,
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

    pub fn draw(
        &mut self,
        delta: f32,
        speed: f32,
        offset_y: f32,
        sinus: bool,
        color: Color,
        rainbow: bool,
    ) {
        let rainbow_colors: Vec<Color> = vec![RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, PINK];
        let mut rainbow_color = rainbow_colors.iter().cycle();
        let letters = self.split_text();
        let mut letter_posx = self.scroll_posx;
        let mut letter_posy;
        for letter in letters {
            if sinus {
                letter_posy = screen_height() / 6.0 * (0.006 * letter_posx).sin() + offset_y;
            } else {
                letter_posy = offset_y;
            }
            let dimtext = measure_text(&letter, Option::Some(self.font), self.font_size, 1.0);
            if rainbow {
                self.draw_letter(
                    letter,
                    Vec2::new(letter_posx, letter_posy),
                    rainbow_color.next().unwrap().to_owned(),
                );
            } else {
                self.draw_letter(letter, Vec2::new(letter_posx, letter_posy), color);
            }
            letter_posx += dimtext.width;
        }
        self.scroll_posx -= speed * delta;
        if letter_posx < -screen_width() / 2.0 {
            self.scroll_posx = screen_width() / 2.0;
        }
    }

    fn draw_letter(&self, letter: String, pos: Vec2, color: Color) {
        let center = center();
        let pos = center + pos;
        draw_text_ex(
            &letter,
            pos.x,
            pos.y,
            TextParams {
                font_size: self.font_size,
                font: self.font,
                color,
                ..Default::default()
            },
        );
    }
}

#[cfg(test)]
#[allow(clippy::mut_mutex_lock)]
mod tests {
    use super::*;

    #[test]
    fn fake_test() {
        assert_eq!(1, 1);
    }

    #[macroquad::test]
    async fn split_text_test() {
        let text = Text::new("Hello", 100);
        let letters = text.split_text();
        assert_eq!(letters, vec!["H", "e", "l", "l", "o"]);
    }
}
