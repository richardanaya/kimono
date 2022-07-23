use ansi_escapes::CursorMove;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

pub struct Style {
    color: (u8, u8, u8),
    background: (u8, u8, u8),
    padding: (usize, usize, usize, usize),
}

fn create_string_with_char(char: char, length: usize) -> String {
    let mut s = String::new();
    for _ in 0..length {
        s.push(char);
    }
    s
}

impl Style {
    pub const fn new() -> Style {
        Style {
            background: (0, 0, 0),
            color: (255, 255, 255),
            padding: (0, 0, 0, 0),
        }
    }

    pub const fn padding_left(mut self, padding: usize) -> Self {
        self.padding.0 = padding;
        self
    }

    pub const fn padding_top(mut self, padding: usize) -> Self {
        self.padding.1 = padding;
        self
    }

    pub const fn padding_right(mut self, padding: usize) -> Self {
        self.padding.2 = padding;
        self
    }

    pub const fn padding_bottom(mut self, padding: usize) -> Self {
        self.padding.3 = padding;
        self
    }

    pub const fn color(mut self, color: u32) -> Self {
        self.color = ((color >> 16) as u8, (color >> 8) as u8, color as u8);
        self
    }

    pub const fn background(mut self, color: u32) -> Self {
        self.background = ((color >> 16) as u8, (color >> 8) as u8, color as u8);
        self
    }

    pub fn render(&self, text: &str) {
        let width = UnicodeWidthStr::width(text) + self.padding.0 + self.padding.2;
        let top = create_string_with_char(' ', width);
        let bottom = create_string_with_char(' ', width);
        let left = create_string_with_char(' ', self.padding.0);
        let right = create_string_with_char(' ', self.padding.2);
        for _ in 0..self.padding.1 {
            print!(
                "{}{}",
                top.on_truecolor(self.background.0, self.background.1, self.background.2),
                CursorMove::XY(-(width as i16), 1)
            );
        }
        print!(
            "{}",
            left.on_truecolor(self.background.0, self.background.1, self.background.2)
        );
        print!(
            "{}",
            text.truecolor(self.color.0, self.color.1, self.color.2)
                .on_truecolor(self.background.0, self.background.1, self.background.2)
        );
        print!(
            "{}{}",
            right.on_truecolor(self.background.0, self.background.1, self.background.2),
            CursorMove::XY(-(width as i16), 1)
        );
        for _ in 0..self.padding.3 {
            print!(
                "{}{}",
                bottom.on_truecolor(self.background.0, self.background.1, self.background.2),
                CursorMove::XY(-(width as i16), 1)
            );
        }
    }
}
