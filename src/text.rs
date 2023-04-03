use crate::screen::center;
use macroquad::prelude::*;

pub struct Text {
    text: String,
    font_size: u16,
    font: Font,
    scroll_posx: f32,
    speed: f32,
    offset_y: f32,
    sinus: bool,
    color: Color,
    rainbow: bool,
}

pub struct TextBuilder {
    text: String,
    font_size: u16,
    font: Font,
    scroll_posx: f32,
    speed: f32,
    offset_y: f32,
    sinus: bool,
    color: Color,
    rainbow: bool,
}

impl TextBuilder {
    pub fn speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    pub fn offset_y(&mut self, offset_y: f32) -> &mut Self {
        self.offset_y = offset_y;
        self
    }

    pub fn sinus(&mut self, sinus: bool) -> &mut Self {
        self.sinus = sinus;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn rainbow(&mut self, rainbow: bool) -> &mut Self {
        self.rainbow = rainbow;
        self
    }

    pub fn build(&self) -> Text {
        Text {
            text: self.text.clone(),
            font_size: self.font_size,
            font: self.font,
            scroll_posx: self.scroll_posx,
            speed: self.speed,
            offset_y: self.offset_y,
            sinus: self.sinus,
            color: self.color,
            rainbow: self.rainbow,
        }
    }
}

impl Text {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(msg: &str, size: u16) -> TextBuilder {
        let font = include_bytes!("../fonts/origami.ttf");
        let font = load_ttf_font_from_bytes(font).unwrap();

        TextBuilder {
            text: msg.to_string(),
            font_size: size,
            font,
            scroll_posx: screen_width() / 2.0,
            speed: 1.0,
            offset_y: 0.0,
            sinus: false,
            color: WHITE,
            rainbow: false,
        }
    }

    fn split_text(&self) -> Vec<String> {
        let mut letters: Vec<String> = Vec::new();
        for letter in self.text.chars() {
            letters.push(letter.to_string());
        }
        letters
    }

    pub fn draw(&mut self, frame_t: f32) {
        let rainbow_colors: Vec<Color> = vec![RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, PINK];
        let mut rainbow_color = rainbow_colors.iter().cycle();
        let letters = self.split_text();
        let mut letter_posx = self.scroll_posx;
        let mut letter_posy;
        for letter in letters {
            if self.sinus {
                letter_posy = screen_height() / 6.0 * (0.006 * letter_posx).sin() + self.offset_y;
            } else {
                letter_posy = self.offset_y;
            }
            let dimtext = measure_text(&letter, Option::Some(self.font), self.font_size, 1.0);
            if self.rainbow {
                self.draw_letter(
                    letter,
                    Vec2::new(letter_posx, letter_posy),
                    rainbow_color.next().unwrap().to_owned(),
                );
            } else {
                self.draw_letter(letter, Vec2::new(letter_posx, letter_posy), self.color);
            }
            letter_posx += dimtext.width;
        }
        self.scroll_posx -= self.speed * frame_t;
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
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn fake_test() {
        assert_eq!(1, 1);
    }

    // #[macroquad::test]
    fn split_text_test() {
        let text = Text::new("Hello", 100).build();
        let letters = text.split_text();
        assert_eq!(letters, vec!["H", "e", "l", "l", "o"]);
    }
}
