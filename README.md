# kimono

<a href="https://docs.rs/kimono"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A simple terminal css engine inspired by [lipgloss](https://github.com/charmbracelet/lipgloss)

```rust
cargo add kimono
```
# Example

```rust
use kimono::*;

const STYLE: Style = Style::new()
    .top_padding(1)
    .left_padding(1)
    .right_padding(2)
    .bottom_padding(3)
    .color(0xff0000)
    .background(0x00ff00);

fn main() {
    STYLE.render("Hello World!")
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
