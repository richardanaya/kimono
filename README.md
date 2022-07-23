# kimono

<a href="https://docs.rs/kimono"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A simple terminal css engine inspired by [lipgloss](https://github.com/charmbracelet/lipgloss) for [truecolor 24-bit terminals](https://github.com/termstandard/colors#terminal-emulators).

```rust
cargo add kimono
```
# Example
<img width="189" alt="Screen Shot 2022-07-23 at 4 38 47 PM" src="https://user-images.githubusercontent.com/294042/180626222-c070e4a7-020c-43ed-96bc-79ebb28371f4.png">

```rust
use kimono::*;

const STYLE: Style = Style::new()
    .padding_top(1)
    .padding_left(1)
    .padding_right(2)
    .padding_bottom(3)
    .border(1)
    .border_color(0xbc5633)
    .color(0xebdeb8)
    .background(0x407955);

fn main() {
    print!("{}{}", ClearScreen, CursorMove::XY(10, 3));
    STYLE.render("着物");
    print!("{}", CursorMove::XY(-10, 3));
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
