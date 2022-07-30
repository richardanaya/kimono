# kimono

<a href="https://docs.rs/kimono"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A terminal style toolkit inspired by CSS and [lipgloss](https://github.com/charmbracelet/lipgloss) for [truecolor 24-bit terminals](https://github.com/termstandard/colors#terminal-emulators).  Made for elegant TUIs.

<p align="center">
<img width="812" alt="Screen Shot 2022-07-30 at 10 57 29 AM" src="https://user-images.githubusercontent.com/294042/181935810-4b355c78-8838-438b-9aca-cdba235aadb6.png">
</p>

Image terminal rendering done by our sister project [ukiyeo](https://github.com/richardanaya/ukiyoe/).

```terminal
cargo add kimono
```
# Examples

*Text can be styled.*

```rust
use kimono::*;

const STYLE: Style = Style::new().bold().color(0x91b984);

fn main() {
    STYLE.render("こんにちは");
}
```

*Unicode text can be padded, bordered, and positioned.*

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
    clear_screen();
    STYLE.render_at_position(10, 3, "着物");
    print!("\n\r");
}
```

*Text can be constrained to width and/or height.*

<img width="188" alt="Screen Shot 2022-07-23 at 9 17 45 PM" src="https://user-images.githubusercontent.com/294042/180631984-110f096e-1b55-4a03-9e54-5e178a13034f.png">

```rust
use kimono::*;

const STYLE: Style = Style::new()
    .padding(1)
    .color(0xfffd7c)
    .width(8)
    .background(0x956471);

fn main() {
    clear_screen();
    STYLE.render_at_position(10, 3, "abcdefghijklmno");
    print!("\n\r");
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

*Borders have advanced styling.*

<img width="340" alt="Screen Shot 2022-07-24 at 1 31 08 PM" src="https://user-images.githubusercontent.com/294042/180664733-0a9fcfb9-0129-40fe-96f9-a02426edb594.png">

```rust
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
```

# Art

Kimono patterns inspired from Unix terminals.

![1658677463667](https://user-images.githubusercontent.com/294042/180660263-281711c1-d29d-48f2-bee4-32b8f0658fd7.jpeg)

![1658677463603](https://user-images.githubusercontent.com/294042/180660265-1a82172d-1b89-4ab6-b653-b496f2396b24.jpeg)

![1658677461856](https://user-images.githubusercontent.com/294042/180660271-d5085b4d-bd4e-463c-acb5-5ed425b92403.jpeg)

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
