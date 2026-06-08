use crate::dates::WithTimeZone;
use axum::{extract::Request, middleware::Next, response::Response};
use chrono::{Datelike, NaiveDate};
use fluent_templates::Loader;
use fluent_templates::fluent_bundle::FluentValue;
use icu::datetime::DateTimeFormatter;
use std::borrow::Cow;
use std::collections::HashMap;
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

pub trait Translator: WithTimeZone {
    fn language(&self) -> Language;

    fn translate(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.language().0, text_id)
    }

    fn translate_with_args(
        &self,
        text_id: &str,
        args: &HashMap<Cow<'static, str>, FluentValue>,
    ) -> String {
        LOCALES.lookup_with_args(&self.language().0, text_id, args)
    }

    fn translate_relative_date(&self, date: &NaiveDate) -> String {
        let today = self.today();
        let yesterday = self.previous_day(&today);
        let tomorrow = self.next_day(&today);

        match date {
            _today if today.eq(_today) => self.translate("today"),
            _yesterday if yesterday.eq(_yesterday) => self.translate("yesterday"),
            _tomorrow if tomorrow.eq(_tomorrow) => self.translate("tomorrow"),

            date => {
                let Language(locale_identifier) = self.language();

                let locale: icu::locale::Locale = locale_identifier
                    .to_string()
                    .parse()
                    .expect("Locale should be valid anyway");

                let formatter_fieldsets = icu::datetime::fieldsets::YMDE::long();

                let date_formatter = DateTimeFormatter::try_new(locale.into(), formatter_fieldsets)
                    .expect("date formatter should be valid anyway");

                let icu_date = icu::calendar::Date::try_new_iso(
                    date.year(),
                    date.month() as u8,
                    date.day() as u8,
                )
                .expect("date should be valid anyway");

                date_formatter.format(&icu_date).to_string()
            }
        }
    }
}
