#[cfg(test)]
mod kimono_test;

#[derive(Debug)]
pub struct StyleSheet {}

impl StyleSheet {
    pub fn from_string(_s: &str) -> Self {
        StyleSheet {}
    }
}

#[derive(Debug, PartialEq)]
enum StyleValue {
    Text(String),
}

impl StyleSheet {
    fn evaluate(&self, _path: &str, _prop: &str) -> Option<StyleValue> {
        Some(StyleValue::Text("red".to_string()))
    }
}
