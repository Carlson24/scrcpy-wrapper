use crate::config_enum;
use std::sync::RwLock;

config_enum! {
    pub enum Language {
        Zh: "zh", "简体中文",
        #[default]
        En: "en", "English",
    }
}

pub static LANGUAGE: RwLock<Language> = RwLock::new(Language::En);
