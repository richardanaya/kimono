pub use ansi_escapes::*;
use colored::ColoredString;
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
    pub top_left: Option<char>,
    pub top: Option<char>,
    pub top_right: Option<char>,
    pub left: Option<char>,
    pub right: Option<char>,
    pub bottom_left: Option<char>,
    pub bottom: Option<char>,
    pub bottom_right: Option<char>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
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
    bold: false,
    italic: false,
    underline: false,
    strikethrough: false,
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
    bold: false,
    italic: false,
    underline: false,
    strikethrough: false,
};

pub const BORDER_STYLE_THICK_OUTLINE: BorderStyle = BorderStyle {
    top_left: Some('┌'),
    top: Some('─'),
    top_right: Some('┐'),
    left: Some('│'),
    right: Some('│'),
    bottom_left: Some('└'),
    bottom: Some('─'),
    bottom_right: Some('┘'),
    bold: true,
    italic: false,
    underline: false,
    strikethrough: false,
};

pub struct Style {
    color: Option<Color>,
    background: Option<Color>,
    padding: Quad,
    border: Quad,
    border_color: Option<Color>,
    border_background: Option<Color>,
    width: Option<usize>,
    height: Option<usize>,
    border_style: Option<BorderStyle>,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
}

fn style_str(
    text: &str,
    color: Option<Color>,
    background: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
) -> ColoredString {
    let mut result: ColoredString = text.into();
    if let Some(color) = color {
        result = result.truecolor(color.0, color.1, color.2);
    }
    if let Some(background) = background {
        result = result.on_truecolor(background.0, background.1, background.2);
    }
    if bold {
        result = result.bold();
    }
    if italic {
        result = result.italic();
    }
    if underline {
        result = result.underline();
    }
    if strikethrough {
        result = result.strikethrough();
    }
    result
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
            background: None,
            color: None,
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
            border_color: None,
            border_background: None,
            width: None,
            height: None,
            border_style: None,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
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
        self.color = Some(((color >> 16) as u8, (color >> 8) as u8, color as u8));
        self
    }

    pub const fn background(mut self, color: u32) -> Self {
        self.background = Some(((color >> 16) as u8, (color >> 8) as u8, color as u8));
        self
    }

    pub const fn border_color(mut self, color: u32) -> Self {
        self.border_color = Some(((color >> 16) as u8, (color >> 8) as u8, color as u8));
        self
    }

    pub const fn border_background(mut self, color: u32) -> Self {
        self.border_background = Some(((color >> 16) as u8, (color >> 8) as u8, color as u8));
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

    fn prepare_text(&self, text: &str) -> (Vec<String>, usize) {
        let mut text: Vec<String> = text
            .replace("\t", "    ")
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        if let Some(w) = self.width {
            let mut max_content_width =
                w - self.padding.left - self.padding.right - self.border.left - self.border.right;
            max_content_width = max_content_width.max(0);
            if max_content_width == 0 {
                text = vec![];
            } else {
                let mut new_lines = vec![];
                {
                    for line in text.iter_mut() {
                        let line_width = get_unicode_length(line);
                        if line_width > max_content_width {
                            let mut new_line = String::new();
                            let mut line_width = 0;
                            for c in line.chars() {
                                let c_width = get_unicode_length(&c.to_string());
                                if line_width + c_width > max_content_width {
                                    new_lines.push(new_line);
                                    new_line = String::new();
                                    line_width = 0;
                                }
                                new_line.push(c);
                                line_width += c_width;
                            }
                            new_lines.push(new_line);
                            break;
                        } else {
                            new_lines.push(line.clone());
                        }
                    }
                }
                text = new_lines;
            }
        }
        if let Some(h) = self.height {
            let mut max_content_height =
                h - self.border.top - self.border.bottom - self.padding.top - self.padding.bottom;
            max_content_height = max_content_height.max(0);
            text = text.iter().take(max_content_height).cloned().collect();
        }
        let mut max_len = text
            .iter()
            .fold(0, |max, line| max.max(get_unicode_length(line)));
        if let Some(w) = self.width {
            max_len =
                w - self.border.left - self.border.right - self.padding.left - self.padding.right;
        }
        (text, max_len)
    }

    pub fn render(&self, text: &str) {
        let (text, max_len) = self.prepare_text(text);
        let width = max_len + self.padding.left + self.padding.right;
        let full_width = width + self.border.left + self.border.right;
        let top = style_str(
            &create_string_with_char(' ', width),
            None,
            self.background,
            false,
            false,
            false,
            false,
        );
        let bottom = style_str(
            &create_string_with_char(' ', width),
            None,
            self.background,
            false,
            false,
            false,
            false,
        );
        let left = style_str(
            &create_string_with_char(' ', self.padding.left),
            None,
            self.background,
            false,
            false,
            false,
            false,
        );
        let right = style_str(
            &create_string_with_char(' ', self.padding.right),
            None,
            self.background,
            false,
            false,
            false,
            false,
        );

        let current_border_style = self.border_style.as_ref().unwrap_or(&BORDER_STYLE_DEFAULT);

        let border_top = if let Some(b) = current_border_style.top {
            style_str(
                &create_string_with_char(b, width),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(width as i16));
            s.into()
        };
        let border_bottom = if let Some(b) = current_border_style.bottom {
            style_str(
                &create_string_with_char(b, width),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(width as i16));
            s.into()
        };
        let border_left = if let Some(b) = current_border_style.left {
            style_str(
                &create_string_with_char(b, self.border.left),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(self.border.left as i16));
            s.into()
        };
        let border_right = if let Some(b) = current_border_style.right {
            style_str(
                &create_string_with_char(b, self.border.right),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(self.border.right as i16));
            s.into()
        };
        let border_top_left = if let Some(b) = current_border_style.top_left {
            style_str(
                &create_string_with_char(b, self.border.left),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(self.border.left as i16));
            s.into()
        };

        let border_top_right = if let Some(b) = current_border_style.top_right {
            style_str(
                &create_string_with_char(b, self.border.right),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(self.border.right as i16));
            s.into()
        };

        let border_bottom_left = if let Some(b) = current_border_style.bottom_left {
            style_str(
                &create_string_with_char(b, self.border.left),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(self.border.left as i16));
            s.into()
        };

        let border_bottom_right = if let Some(b) = current_border_style.bottom_right {
            style_str(
                &create_string_with_char(b, self.border.right),
                self.border_color,
                self.border_background,
                current_border_style.bold,
                current_border_style.italic,
                current_border_style.underline,
                current_border_style.strikethrough,
            )
        } else {
            let s: &str = &format!("{}", CursorMove::X(self.border.right as i16));
            s.into()
        };

        // top
        for _ in 0..self.border.top {
            print!("{}", border_top_left);
            print!("{}", border_top);
            print!(
                "{}{}",
                border_top_right,
                CursorMove::XY(-(full_width as i16), 1)
            );
        }
        for _ in 0..self.padding.top {
            print!("{}", border_left);
            print!("{}", top);
            print!(
                "{}{}",
                border_right,
                CursorMove::XY(-(full_width as i16), 1)
            );
        }

        // content
        for line in text.iter() {
            print!("{}", border_left);
            print!("{}", left);
            let padding_len = max_len - get_unicode_length(line);
            let padding = create_string_with_char(' ', padding_len);
            print!(
                "{}{}",
                style_str(
                    &line,
                    self.color,
                    self.background,
                    self.bold,
                    self.italic,
                    self.underline,
                    self.strikethrough
                ),
                style_str(
                    &padding,
                    self.color,
                    self.background,
                    false,
                    false,
                    false,
                    false
                )
            );
            print!("{}", right,);
            print!(
                "{}{}",
                border_right,
                CursorMove::XY(-(full_width as i16), 1)
            );
        }

        if let Some(h) = self.height {
            let max_content_height =
                (h - self.border.top - self.padding.top - self.padding.bottom).max(0);
            let excess_lines = (max_content_height - text.len()).max(0);

            let padding = create_string_with_char(' ', max_len);
            for _ in 0..excess_lines {
                print!("{}", border_left);
                print!("{}", left);
                print!(
                    "{}",
                    style_str(
                        &padding,
                        self.color,
                        self.background,
                        false,
                        false,
                        false,
                        false
                    )
                );
                print!("{}", right,);
                print!(
                    "{}{}",
                    border_right,
                    CursorMove::XY(-(full_width as i16), 1)
                );
            }
        }

        // bottom
        for _ in 0..self.padding.bottom {
            print!("{}", border_left);
            print!("{}", bottom);
            print!(
                "{}{}",
                border_right,
                CursorMove::XY(-(full_width as i16), 1)
            );
        }
        for _ in 0..self.border.bottom {
            print!("{}", border_bottom_left);
            print!("{}", border_bottom);
            print!(
                "{}{}",
                border_bottom_right,
                CursorMove::XY(-(full_width as i16), 1)
            );
        }
    }

    pub fn render_at_position(&self, x: u16, y: u16, text: &str) {
        print!("{}", CursorTo::AbsoluteXY(x, y));
        self.render(text)
    }

    pub fn measure(&self, text: &str) -> (usize, usize) {
        let (text, max_width) = self.prepare_text(text);
        let cols = max_width
            + self.padding.left
            + self.padding.right
            + self.border.left
            + self.border.right;
        let rows = text.len()
            + self.padding.top
            + self.padding.bottom
            + self.border.top
            + self.border.bottom;
        (cols, rows)
    }

    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub const fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub const fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub const fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }
}

pub fn clear_screen() {
    print!("{}", CursorMove::XY(0, 0));
    print!("{}", EraseScreen);
}
