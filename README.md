# kimono

<a href="https://docs.rs/kimono"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A simple terminal css engine inspired by [lipgloss](https://github.com/charmbracelet/lipgloss)

```rust
cargo add kimono
```
# Example

```rust
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
```
<img width="377" alt="Screen Shot 2022-07-23 at 2 23 28 PM" src="https://user-images.githubusercontent.com/294042/180623295-f0a7f94f-19b1-476d-9d48-04c29478911e.png">

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
