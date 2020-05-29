# kimono

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