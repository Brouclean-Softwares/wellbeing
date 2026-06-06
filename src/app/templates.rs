use crate::AppState;
use crate::data::users::{ConnectedProfile, Profile};
use crate::languages::Translator;
use askama::Template;
use askama_web::WebTemplate;
use std::borrow::Cow;
use std::collections::HashMap;

pub mod satisfying_moments;
pub mod users;

#[derive(Template, WebTemplate)]
#[template(path = "welcome_page.html")]
pub struct WelcomePage {
    navigation_bar: NavigationBar,
    profile: Profile,
}

impl WelcomePage {
    pub fn get(app_state: &AppState, profile: &Profile) -> Self {
        Self {
            navigation_bar: NavigationBar::get_from_profile(app_state, &profile),
            profile: profile.clone(),
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "home_page.html")]
pub struct HomePage {
    navigation_bar: NavigationBar,
    welcome_user_translation: String,
}

impl HomePage {
    pub fn get(app_state: &AppState, connected_profile: &ConnectedProfile) -> Self {
        let welcome_user_translation = connected_profile.translate_with_args(
            "welcome_name",
            &HashMap::from([(
                Cow::from("name"),
                connected_profile.user.clone().given_name.into(),
            )]),
        );

        Self {
            navigation_bar: NavigationBar::get_from_connected_profile(
                app_state,
                &connected_profile,
            ),
            welcome_user_translation,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "navigation_bar.html")]
pub struct NavigationBar {
    profile: Profile,
    user_is_admin: bool,
}

impl NavigationBar {
    pub fn get_from_profile(app_state: &AppState, profile: &Profile) -> Self {
        let user_is_admin = match &profile.user {
            Some(user) => user.is_admin(app_state),
            _ => false,
        };

        Self {
            profile: profile.clone(),
            user_is_admin,
        }
    }
    pub fn get_from_connected_profile(
        app_state: &AppState,
        connected_profile: &ConnectedProfile,
    ) -> Self {
        Self::get_from_profile(app_state, &Profile::from(connected_profile.clone()))
    }
}
