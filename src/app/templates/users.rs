use crate::AppState;
use crate::Locale;
use crate::app::templates::NavigationBar;
use crate::data::sessions::Session;
use crate::data::users::{ConnectedProfile, User};
use crate::locales::Translator;
use askama::Template;
use askama_web::WebTemplate;
use unic_langid::LanguageIdentifier;

#[derive(Template, WebTemplate)]
#[template(path = "users/users_page.html")]
pub struct UsersPage {
    navigation_bar: NavigationBar,
    connected_profile: ConnectedProfile,
    language_identifier: LanguageIdentifier,
    sessions_with_entries_count: Vec<(Session, i64)>,
}

impl UsersPage {
    pub fn from(
        connected_profile: ConnectedProfile,
        sessions_with_entries_count: Vec<(Session, i64)>,
    ) -> Self {
        let Locale(language_identifier) = connected_profile.language.clone();

        Self {
            navigation_bar: NavigationBar::from(connected_profile.clone()),
            connected_profile,
            language_identifier,
            sessions_with_entries_count,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "users/user_page.html")]
pub struct UserPage {
    navigation_bar: NavigationBar,
    connected_profile: ConnectedProfile,
    user: User,
    user_is_admin: bool,
}

impl UserPage {
    pub fn from(app_state: AppState, connected_profile: ConnectedProfile, user: User) -> Self {
        let user_is_admin = user.is_admin(&app_state);

        Self {
            navigation_bar: NavigationBar::from(connected_profile.clone()),
            connected_profile,
            user,
            user_is_admin,
        }
    }
}
