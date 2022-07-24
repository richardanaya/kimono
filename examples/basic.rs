use kimono::*;

const STYLE: Style = Style::new()
    .padding(1)
    .color(0xfffd7c)
    .width(8)
    .background(0x956471);

fn main() {
    let text = "abcdefghijklmno";
    print!("{}{}", ClearScreen, CursorMove::XY(10, 3));
    STYLE.render(text);
    print!("{}", CursorMove::XY(-10, 3));
    println!("{:?}", STYLE.measure(text));
}
