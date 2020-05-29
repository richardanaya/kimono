use crate::*;

#[test]
fn it_loads_basic_style() {
    let styles = Kimono::from_string(
        r#"
        foo {
            color: red
        }
    "#,
    );
    assert_eq!(styles.lookup("foo.color"), "red");
}
