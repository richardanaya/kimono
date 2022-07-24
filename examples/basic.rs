use kimono::*;

const STYLE: Style = Style::new()
    .padding(1)
    .color(0xe46281)
    .background(0xc50f47)
    .border(1)
    .italic()
    .border_style(BorderStyle {
        top_left: None,
        top: Some('•'),
        top_right: None,
        left: Some('•'),
        right: Some('•'),
        bottom_left: None,
        bottom: Some('•'),
        bottom_right: None,
        bold: true,
        italic: false,
        underline: false,
        strikethrough: false,
    })
    .border_color(0xe5c7c9)
    .border_background(0x9e1d49);

fn main() {
    clear_screen();
    STYLE.render_at_position(10, 3, "The Tale of Genji by 紫 式部");
    print!("\n\r");
}
