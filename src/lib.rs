#[cfg(test)]
mod kimono_test;

trait StyleSheet {
    fn evaluate(&self, path: &str, prop: &str) -> Option<StyleValue> ;
}

#[derive(Debug)]
pub struct Kimono {}

impl Kimono {
    pub fn from_string(_s: &str) -> Self {
        Kimono {}
    }
}

#[derive(Debug,PartialEq)]
enum StyleValue {
    Text(String)
}

impl StyleSheet for Kimono {
    fn evaluate(&self, _path: &str, _prop: &str) -> Option<StyleValue> {
        Some(StyleValue::Text("red".to_string()))
    }
}
