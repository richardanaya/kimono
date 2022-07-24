# kimono

<a href="https://docs.rs/kimono"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A terminal style toolkit inspired by CSS and [lipgloss](https://github.com/charmbracelet/lipgloss) for [truecolor 24-bit terminals](https://github.com/termstandard/colors#terminal-emulators).

```terminal
cargo add kimono
```
# Example

*Unicode text can be styled, padded, and bordered.*

<img width="180" alt="Screen Shot 2022-07-23 at 5 01 28 PM" src="https://user-images.githubusercontent.com/294042/180626676-67a67dff-25fa-4deb-8cc9-683e17ca9a64.png">

```rust
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
    STYLE.render("着物");
    print!("{}", CursorMove::XY(-10, 3));
}
```

*Text can be constrained to height and width.*

<img width="188" alt="Screen Shot 2022-07-23 at 9 17 45 PM" src="https://user-images.githubusercontent.com/294042/180631984-110f096e-1b55-4a03-9e54-5e178a13034f.png">

```rust
use kimono::*;

const STYLE: Style = Style::new()
    .padding(1)
    .color(0xfffd7c)
    .width(8)
    .background(0x956471);

fn main() {
    print!("{}{}", ClearScreen, CursorMove::XY(10, 3));
    STYLE.render(text);
    print!("{}", CursorMove::XY(-10, 3));
    println!("{:?}", STYLE.measure("abcdefghijklmno"));
}
```

*Text can be measured.*

```terminal
(8, 5)
```

```rust
use kimono::*;

const STYLE: Style = Style::new()
    .padding(1)
    .color(0xfffd7c)
    .width(8)
    .background(0x956471);

fn main() {
    println!("{:?}", STYLE.measure("abcdefghijklmno"));
}
```

*Borders have advanced styling*

<img width="330" alt="Screen Shot 2022-07-24 at 10 53 02 AM" src="https://user-images.githubusercontent.com/294042/180659763-273cc486-ecee-43dc-bff0-0fede6a5a219.png">

```rust
use kimono::*;

const STYLE: Style = Style::new()
    .padding(1)
    .color(0xe46281)
    .background(0xc50f47)
    .border(1)
    .italic()
    .border_style(BorderStyle {
        top_left: Some(' '),
        top: Some('•'),
        top_right: Some(' '),
        left: Some('•'),
        right: Some('•'),
        bottom_left: Some(' '),
        bottom: Some('•'),
        bottom_right: Some(' '),
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
    println!("\n\r");
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `kimono` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
