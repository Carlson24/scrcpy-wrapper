use std::sync::RwLock;

#[derive(Clone, Debug)]
pub enum Language {
    Zh,
    En,
}

pub static LANGUAGE: RwLock<Language> = RwLock::new(Language::En);
