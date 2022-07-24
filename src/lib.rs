pub use ansi_escapes::*;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

type Color = (u8, u8, u8);
struct Quad {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

#[derive(Default)]
pub struct BorderStyle {
    top_left: Option<char>,
    top: Option<char>,
    top_right: Option<char>,
    left: Option<char>,
    right: Option<char>,
    bottom_left: Option<char>,
    bottom: Option<char>,
    bottom_right: Option<char>,
}

pub const BORDER_STYLE_DEFAULT: BorderStyle = BorderStyle {
    top_left: Some(' '),
    top: Some(' '),
    top_right: Some(' '),
    left: Some(' '),
    right: Some(' '),
    bottom_left: Some(' '),
    bottom: Some(' '),
    bottom_right: Some(' '),
};

pub const BORDER_STYLE_OUTLINE: BorderStyle = BorderStyle {
    top_left: Some('┌'),
    top: Some('─'),
    top_right: Some('┐'),
    left: Some('│'),
    right: Some('│'),
    bottom_left: Some('└'),
    bottom: Some('─'),
    bottom_right: Some('┘'),
};

pub struct Style {
    color: Color,
    background: Color,
    padding: Quad,
    border: Quad,
    border_color: Color,
    border_background: Color,
    width: Option<usize>,
    height: Option<usize>,
    border_style: Option<BorderStyle>,
}

fn create_string_with_char(char: char, length: usize) -> String {
    let mut s = String::new();
    for _ in 0..length {
        s.push(char);
    }
    s
}

fn get_unicode_length(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

impl Style {
    pub const fn new() -> Style {
        Style {
            background: (0, 0, 0),
            color: (255, 255, 255),
            padding: Quad {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            border: Quad {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            border_color: (255, 255, 255),
            border_background: (0, 0, 0),
            width: None,
            height: None,
            border_style: None,
        }
    }

    pub const fn border_style(mut self, border_style: BorderStyle) -> Style {
        self.border_style = Some(border_style);
        self
    }

    pub const fn padding(mut self, padding: usize) -> Self {
        self.padding = Quad {
            left: padding,
            top: padding,
            right: padding,
            bottom: padding,
        };
        self
    }

    pub const fn padding_left(mut self, padding: usize) -> Self {
        self.padding.left = padding;
        self
    }

    pub const fn padding_top(mut self, padding: usize) -> Self {
        self.padding.top = padding;
        self
    }

    pub const fn padding_right(mut self, padding: usize) -> Self {
        self.padding.right = padding;
        self
    }

    pub const fn padding_bottom(mut self, padding: usize) -> Self {
        self.padding.bottom = padding;
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

    pub const fn border_color(mut self, color: u32) -> Self {
        self.border_color = ((color >> 16) as u8, (color >> 8) as u8, color as u8);
        self
    }

    pub const fn border_background(mut self, color: u32) -> Self {
        self.border_background = ((color >> 16) as u8, (color >> 8) as u8, color as u8);
        self
    }

    pub const fn border(mut self, border: usize) -> Self {
        self.border = Quad {
            left: border,
            top: border,
            right: border,
            bottom: border,
        };
        self
    }

    pub const fn border_left(mut self, left: usize) -> Self {
        self.border.left = left;
        self
    }

    pub const fn border_top(mut self, top: usize) -> Self {
        self.border.top = top;
        self
    }

    pub const fn border_right(mut self, right: usize) -> Self {
        self.border.right = right;
        self
    }

    pub const fn border_bottom(mut self, bottom: usize) -> Self {
        self.border.bottom = bottom;
        self
    }

    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub const fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn render(&self, text: &str) {
        let text: Vec<String> = text
            .replace("\t", "    ")
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        let max_len = text
            .iter()
            .fold(0, |max, line| max.max(get_unicode_length(line)));
        let width = max_len + self.padding.left + self.padding.right;
        let full_width = width + self.border.left + self.border.right;
        let top = create_string_with_char(' ', width);
        let bottom = create_string_with_char(' ', width);
        let left = create_string_with_char(' ', self.padding.left);
        let right = create_string_with_char(' ', self.padding.right);

        let border_top = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .top
                .unwrap_or(' '),
            width,
        );
        let border_bottom = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .bottom
                .unwrap_or(' '),
            width,
        );
        let border_left = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .left
                .unwrap_or(' '),
            self.border.left,
        );
        let border_right = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .right
                .unwrap_or(' '),
            self.border.right,
        );
        let border_top_left = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .top_left
                .unwrap_or(' '),
            self.border.left,
        );
        let border_top_right = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .top_right
                .unwrap_or(' '),
            self.border.right,
        );
        let border_bottom_left = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .bottom_left
                .unwrap_or(' '),
            self.border.left,
        );
        let border_bottom_right = create_string_with_char(
            self.border_style
                .as_ref()
                .unwrap_or(&BORDER_STYLE_DEFAULT)
                .bottom_right
                .unwrap_or(' '),
            self.border.right,
        );

        // top
        for _ in 0..self.border.top {
            print!(
                "{}",
                border_top_left
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}",
                border_top
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}{}",
                border_top_right
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    ),
                CursorMove::XY(-(full_width as i16), 1)
            );
        }
        for _ in 0..self.padding.top {
            print!(
                "{}",
                border_left
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}",
                top.on_truecolor(self.background.0, self.background.1, self.background.2)
            );
            print!(
                "{}{}",
                border_right
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    ),
                CursorMove::XY(-(full_width as i16), 1)
            );
        }

        // content
        for line in text.iter() {
            print!(
                "{}",
                border_left
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}",
                left.on_truecolor(self.background.0, self.background.1, self.background.2)
            );
            let padding_len = max_len - get_unicode_length(line);
            let padding = create_string_with_char(' ', padding_len);
            print!(
                "{}{}",
                line.truecolor(self.color.0, self.color.1, self.color.2)
                    .on_truecolor(self.background.0, self.background.1, self.background.2),
                padding.truecolor(self.color.0, self.color.1, self.color.2)
                .on_truecolor(self.background.0, self.background.1, self.background.2)
            );
            print!(
                "{}",
                right.on_truecolor(self.background.0, self.background.1, self.background.2),
            );
            print!(
                "{}{}",
                border_right
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    ),
                CursorMove::XY(-(full_width as i16), 1)
            );
        }

        // bottom
        for _ in 0..self.padding.bottom {
            print!(
                "{}",
                border_left
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}",
                bottom.on_truecolor(self.background.0, self.background.1, self.background.2)
            );
            print!(
                "{}{}",
                border_right
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    ),
                CursorMove::XY(-(full_width as i16), 1)
            );
        }
        for _ in 0..self.border.bottom {
            print!(
                "{}",
                border_bottom_left
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}",
                border_bottom
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    )
            );
            print!(
                "{}{}",
                border_bottom_right
                    .truecolor(
                        self.border_color.0,
                        self.border_color.1,
                        self.border_color.2
                    )
                    .on_truecolor(
                        self.border_background.0,
                        self.border_background.1,
                        self.border_background.2
                    ),
                CursorMove::XY(-(full_width as i16), 1)
            );
        }
    }
}
