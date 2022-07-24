use kimono::*;

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
    .width(38)
    .color(0xffffff);

fn main() {
    clear_screen();
    TAB_HOLDER.render_at_position(0, 2, "Hello world, test\nfads\nsdsd");
    TAB_SELECTED.render_at_position(1, 0, "Kimono");
    TAB_UNSELECTED.render_at_position(9, 0, "Shoji");
    TAB_UNSELECTED.render_at_position(16, 0, "Mochi");
    TAB_UNSELECTED.render_at_position(23, 0, "Ramen");
    TAB_UNSELECTED.render_at_position(30, 0, "Samurai");
    print!("\n\r");
}
