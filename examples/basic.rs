use ansi_escapes::*;
use kimono::*;

const STYLE: Style = Style::new()
    .top_padding(1)
    .left_padding(1)
    .right_padding(2)
    .bottom_padding(3)
    .color(0xebdeb8)
    .background(0x407955);

fn main() {
    print!("{}{}", ClearScreen, CursorMove::XY(10, 3));
    STYLE.render("着物");
    print!("{}", CursorMove::XY(-10, 3));
}
