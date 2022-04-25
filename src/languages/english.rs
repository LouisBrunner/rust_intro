use super::traits;

pub struct English;

impl English {
    pub fn new() -> Self {
        Self {}
    }
}

impl traits::Language for English {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}
