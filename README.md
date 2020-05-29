# kimono

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
        color: red;
    }

    #magic_button {
        font-size: 12px;
    }

    .bold {
        font-weight: bold;
    }
"#);

// match by type
assert_eq!(styles.evaluate(Some("foo"),None,None,"color"), Some("red"));
// match by id
assert_eq!(styles.evaluate(None,Some("#magic_button"),None,"font-size"), Some("black"));
// match by class
assert_eq!(styles.evaluate(Some("bar"),None,Some(".bold"),"font-weight"), Some("bold"));
// handle match fail
assert_eq!(styles.evaluate(None,Some("#random"),None,"random_prop"), None);
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
