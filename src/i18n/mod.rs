#[cfg(test)]
use std::sync::Once;
use std::{path::Path, sync::OnceLock};

use i18n_embed::{FileSystemAssets, LanguageLoader, fluent::FluentLanguageLoader, unic_langid};

use super::modes::KEALintLanguages;

static LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub struct I18N;

impl I18N {
    pub fn new(lang: KEALintLanguages) -> Result<Self, Box<dyn std::error::Error>> {
        let fallback = "en".parse()?;

        let loader = FluentLanguageLoader::new("kealint", fallback);
        let assets = FileSystemAssets::try_new(Path::new("locales"))?;

        loader.load_available_languages(&assets)?;

        let language: unic_langid::LanguageIdentifier = lang.to_string().parse()?;

        loader.load_languages(&assets, &[language])?;

        LOADER
            .set(loader)
            .map_err(|_| "i18n loader already initialized.")?;

        Ok(Self)
    }
}

#[cfg(test)]
static TEST_INIT_LOADER: Once = Once::new();

pub fn loader() -> &'static FluentLanguageLoader {
    #[cfg(test)]
    {
        if LOADER.get().is_none() {
            TEST_INIT_LOADER.call_once(|| {
                if LOADER.get().is_none() {
                    I18N::new(KEALintLanguages::en).unwrap();
                }
            });
        }
    }

    LOADER.get().expect("i18n is not initialized")
}

#[macro_export]
macro_rules! tr {
    ($key:literal) => {
        i18n_embed_fl::fl!($crate::i18n::loader(), $key)
    };

    ($key:literal, $($args:tt)*) => {
        i18n_embed_fl::fl!($crate::i18n::loader(), $key, $($args)*)
    };
}
