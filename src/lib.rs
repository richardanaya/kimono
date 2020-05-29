#[cfg(test)]
mod kimono_test;

trait StyleSheet {
    fn lookup(&self, path: &str) -> String;
}

pub struct Kimono {}

impl Kimono {
    pub fn from_string(_s: &str) -> Self {
        Kimono {}
    }
}

enum StyleValue {

}

impl StyleSheet for Kimono {
    fn evaluate(&self, _path: &str) -> StyleValue {
        "red".to_string()
    }
}
