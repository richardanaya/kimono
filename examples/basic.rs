use kimono::*;

const STYLE: Style = Style::new()
    .padding_top(1)
    .padding_left(1)
    .padding_right(2)
    .padding_bottom(3)
    .border(1)
    .border_style(BORDER_STYLE_OUTLINE)
    .border_color(0xddae74)
    .border_background(0xbc5633)
    .color(0xebdeb8)
    .background(0x407955);

fn main() {
    print!("{}{}", ClearScreen, CursorMove::XY(10, 3));
    STYLE.render("着\t物\nkimono");
    print!("{}", CursorMove::XY(-10, 3));
}
