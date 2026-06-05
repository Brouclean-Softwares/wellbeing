use axum::{extract::Request, middleware::Next, response::Response};
use fluent_templates::Loader;
use std::iter::Iterator;
use unic_langid::{LanguageIdentifier, langid};

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

fn map_lang_to_supported_language(lang: &str) -> Option<LanguageIdentifier> {
    match lang.split('-').next() {
        Some("en") => Some(langid!("en-US")),
        Some("fr") => Some(langid!("fr-FR")),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub struct Language(pub LanguageIdentifier);

impl Language {
    fn from_browser_accepted_languages(browser_accepted_languages: Vec<String>) -> Self {
        for browser_accepted_language in browser_accepted_languages {
            if let Some(language) = LOCALES
                .locales()
                .find(|&lang| lang.to_string().eq(&browser_accepted_language))
            {
                return Self(language.clone());
            } else {
                let related_language =
                    map_lang_to_supported_language(browser_accepted_language.as_str());

                if let Some(language) = related_language {
                    return Self(language.into());
                }
            }
        }

        Self(LOCALES.fallback().clone())
    }

    pub fn translate(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.0, text_id)
    }

    pub fn accepted_languages() -> Vec<Self> {
        LOCALES
            .locales()
            .map(|locale| Self(locale.clone()))
            .collect()
    }

    pub async fn detect_language(request: Request, next: Next) -> Result<Response, Response> {
        let headers = request.headers();

        let browser_accepted_languages = accept_language::parse(
            headers
                .get("Accept-Language")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(&LOCALES.fallback().to_string()),
        );

        let preferred_lang = Self::from_browser_accepted_languages(browser_accepted_languages);

        let mut request_with_preferred_lang = request;
        request_with_preferred_lang
            .extensions_mut()
            .insert(preferred_lang);

        Ok(next.run(request_with_preferred_lang).await)
    }
}
