use kimono::*;
use ukiyoe::Image;

const TAB_SELECTED: Style = Style::new()
    .color(0x0d9211)
    .border(1)
    .italic()
    .border_style(BorderStyle {
        top_left: Some('╭'),
        top: Some('─'),
        top_right: Some('╮'),
        left: Some('│'),
        right: Some('│'),
        bottom_left: Some('╯'),
        bottom: Some(' '),
        bottom_right: Some('╰'),
        bold: true,
        italic: false,
        underline: false,
        strikethrough: false,
    })
    .border_color(0xe5c7c9);

const TAB_UNSELECTED: Style = TAB_SELECTED.border_style(BorderStyle {
    top_left: Some('╭'),
    top: Some('─'),
    top_right: Some('╮'),
    left: Some('│'),
    right: Some('│'),
    bottom_left: Some('┴'),
    bottom: Some('─'),
    bottom_right: Some('┴'),
    bold: true,
    italic: false,
    underline: false,
    strikethrough: false,
});

const TAB_HOLDER: Style = TAB_SELECTED
    .border_style(BorderStyle {
        top_left: Some('╭'),
        top: None,
        top_right: Some('╮'),
        left: Some('│'),
        right: Some('│'),
        bottom_left: Some('╰'),
        bottom: Some('─'),
        bottom_right: Some('╯'),
        bold: true,
        italic: false,
        underline: false,
        strikethrough: false,
    })
    .width(40)
    .color(0xffffff);

const CONTAINER: Style = TAB_SELECTED
    .border_style(BorderStyle {
        top_left: Some('╭'),
        top: Some('─'),
        top_right: Some('╮'),
        left: Some('│'),
        right: Some('│'),
        bottom_left: Some('╰'),
        bottom: Some('─'),
        bottom_right: Some('╯'),
        bold: true,
        italic: false,
        underline: false,
        strikethrough: false,
    })
    .width(40)
    .color(0xffffff);

const FANCY: Style = Style::new().background(0xdcb97e).color(0x0d9211);
const FANCY1: Style = Style::new().background(0xfff1ce).color(0x0d9211);
const FANCY2: Style = Style::new().background(0xfe3900).color(0x0d9211);
const FANCY3: Style = Style::new().background(0x657374).color(0x0d9211);

const FANCY4: Style = Style::new().color(0x0d9211);

const WHITE: Style = Style::new().color(0xdcdcdc);
const SEPARATOR: Style = Style::new().color(0x5f6563);

fn main() {
    clear_screen();
    TAB_HOLDER.render_at_position(0, 2, "Yōkai (妖怪, \"strange apparition\") are a class of supernatural entities and spirits in Japanese folklore. The word yōkai is composed of the kanji for \"attractive; calamity\" and \"apparition; mystery; suspicious.\"[1][2] Yōkai are also referred to as ayakashi (あやかし), mononoke (物の怪) or mamono (魔物). Yokai are not literally demons in the Western sense of the word, but are instead spirits and entities, whose behaviour can range from malevolent or mischievous to friendly, fortuitous, or helpful to humans.");
    TAB_SELECTED.render_at_position(1, 0, "Kimono");
    TAB_UNSELECTED.render_at_position(9, 0, "Shoji");
    TAB_UNSELECTED.render_at_position(16, 0, "Mochi");
    TAB_UNSELECTED.render_at_position(23, 0, "Ramen");
    TAB_UNSELECTED.render_at_position(30, 0, "Samurai");

    let mut image = Image::new("examples/test.png");
    image.render_at_position(40, 0, 60, 18);

    FANCY.render_at_position(65, 8, "KIMONO");
    FANCY1.render_at_position(67, 9, "KIMONO");
    FANCY2.render_at_position(69, 10, "KIMONO");
    FANCY3.render_at_position(71, 11, "KIMONO");

    WHITE.render_at_position(45, 21, "A terminal style library for elegant TUIs");
    FANCY4.render_at_position(45, 22, "https://github.com/richardanaya/kimono");

    CONTAINER.width(100).render_at_position(
        0,
        18,
        "░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░",
    );
    print!("\n\r");
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Please enter the name you seek: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}", s);
}
