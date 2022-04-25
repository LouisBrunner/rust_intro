pub trait Language {
    fn greet(&self, name: &str) -> String;
}
