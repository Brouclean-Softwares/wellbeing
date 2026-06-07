use crate::data::users::{ConnectedProfile, Profile};
use crate::languages::Translator;
use askama::Template;
use askama_web::WebTemplate;
use std::borrow::Cow;
use std::collections::HashMap;

pub mod journal;
pub mod satisfying_moments;
pub mod shared;
pub mod users;

#[derive(Template, WebTemplate)]
#[template(path = "welcome_page.html")]
pub struct WelcomePage {
    navigation_bar: NavigationBar,
    profile: Profile,
}

impl From<Profile> for WelcomePage {
    fn from(profile: Profile) -> Self {
        Self {
            navigation_bar: NavigationBar::from(profile.clone()),
            profile,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "welcome_with_love_page.html")]
pub struct WelcomeWithLovePage {
    navigation_bar: NavigationBar,
    welcome_user_with_love: String,
}

impl From<ConnectedProfile> for WelcomeWithLovePage {
    fn from(connected_profile: ConnectedProfile) -> Self {
        let welcome_user_with_love = connected_profile.translate_with_args(
            "welcome_name_with_love",
            &HashMap::from([(
                Cow::from("name"),
                connected_profile.user.clone().given_name.into(),
            )]),
        );

        Self {
            navigation_bar: NavigationBar {
                profile: connected_profile.into(),
            },
            welcome_user_with_love,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "navigation_bar.html")]
pub struct NavigationBar {
    profile: Profile,
}

impl From<Profile> for NavigationBar {
    fn from(profile: Profile) -> Self {
        Self { profile }
    }
}

impl From<ConnectedProfile> for NavigationBar {
    fn from(connected_profile: ConnectedProfile) -> Self {
        Self::from(Profile::from(connected_profile))
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "unauthorized_page.html")]
pub struct UnauthorizedPage {
    navigation_bar: NavigationBar,
    profile: Profile,
}

impl From<Profile> for UnauthorizedPage {
    fn from(profile: Profile) -> Self {
        Self {
            navigation_bar: NavigationBar::from(profile.clone()),
            profile,
        }
    }
}
