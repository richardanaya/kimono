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

impl StyleSheet for Kimono {
    fn lookup(&self, _path: &str) -> String {
        "red".to_string()
    }
}
