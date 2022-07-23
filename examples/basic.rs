use kimono::*;

const STYLE: Style = Style::new()
    .top_padding(1)
    .left_padding(1)
    .right_padding(2)
    .bottom_padding(3)
    .color(0xff0000)
    .background(0x00ff00);

fn main() {
    STYLE.render("hey!!!!!")
}
