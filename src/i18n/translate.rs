#[macro_export]
macro_rules! t {
    {
        en: $fmt_en:expr,
        zh: $fmt_zh:expr,
        ($($args:tt)*)
    } => {
        *match &*$crate::i18n::LANGUAGE.read().unwrap(){
            $crate::i18n::Language::En => {
                let res = std::fmt::format(format_args!($fmt_en,$($args)*));
                res
            },
            $crate::i18n::Language::Zh => {
                let res = std::fmt::format(format_args!($fmt_zh,$($args)*));
                res
            }
        }
    };
    {
        zh: $fmt_zh:expr,
        en: $fmt_en:expr,
        ($($args:tt)*)
    } => {
        t! {
            en: $fmt_en,
            zh: $fmt_zh,
            ($($args)*)
        }
    };
    {
        zh: $fmt_zh:expr,
        en: $fmt_en:expr
    } => {
        t! {
            en: $fmt_en,
            zh: $fmt_zh,
            ()
        }
    };
    {
        en: $fmt_en:expr,
        zh: $fmt_zh:expr
    } => {
        t! {
            en: $fmt_en,
            zh: $fmt_zh,
            ()
        }
    };
    
    
    {r
        en: $fmt_en:expr,
        zh: $fmt_zh:expr
    } => {
        
        *match &*$crate::i18n::LANGUAGE.read().unwrap(){
            $crate::i18n::Language::En => {
                $fmt_en
            },
            $crate::i18n::Language::Zh => {
                $fmt_zh
            }
        }
    };
    {r
        zh: $fmt_zh:expr,
        en: $fmt_en:expr
    } => {
        t! {r
            en: $fmt_en,
            zh: $fmt_zh
        }
    };
}