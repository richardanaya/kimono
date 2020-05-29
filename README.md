# :kimono: kimono

<a href="https://docs.rs/kimono"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A simple CSS engine for a variety of needs.

- [x] style agnostic 
- [ ] simple typed extraction support
- [ ] ordering matters
- [ ] specificity rules
- [ ] handle multiple files

```rust
let k = Kimono::from_string(r#"
    foo {
        color: red
    }
"#);
assert_eq!(styles.lookup("foo.color"), "red");
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
for inclusion in `watson` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
