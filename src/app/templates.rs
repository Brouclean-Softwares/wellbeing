use crate::AppState;
use crate::data::users::Profile;
use askama::Template;
use askama_web::WebTemplate;
use std::borrow::Cow;
use std::collections::HashMap;

pub mod users;

#[derive(Template, WebTemplate)]
#[template(path = "home_page.html")]
pub struct HomePage {
    navigation_bar: NavigationBar,
    profile: Profile,
    welcome_user_translation: String,
}

impl HomePage {
    pub async fn get(app_state: &AppState, profile: &Profile) -> Self {
        let welcome_user_translation = profile.language.translate_with_args(
            "welcome_name",
            &HashMap::from([(
                Cow::from("name"),
                profile.user.clone().unwrap_or_default().given_name.into(),
            )]),
        );

        Self {
            navigation_bar: NavigationBar::get(app_state, &profile),
            profile: profile.clone(),
            welcome_user_translation,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "navigation_bar.html")]
pub struct NavigationBar {
    profile: Profile,
    is_admin: bool,
}

impl NavigationBar {
    pub fn get(app_state: &AppState, profile: &Profile) -> Self {
        let is_admin = match &profile.user {
            Some(user) => user.is_admin(app_state),
            _ => false,
        };

        Self {
            profile: profile.clone(),
            is_admin,
        }
    }
}
