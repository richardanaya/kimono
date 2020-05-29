use crate::*;

#[test]
fn it_loads_basic_style() {
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
    assert_eq!(k.evaluate("foo","color"), Some(StyleValue::Text("red".to_string())));
}
