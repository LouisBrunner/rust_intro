mod english;
mod traits;

pub enum LanguageName {
    English,
    // TODO: add more languages??
}

pub struct LanguageFactory;

impl LanguageFactory {
    pub fn new_language(name: LanguageName) -> Box<dyn traits::Language> {
        match name {
            LanguageName::English => Box::new(english::English::new()),
        }
    }
}
