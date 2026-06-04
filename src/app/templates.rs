use crate::AppState;
use crate::data::users::User;
use askama::Template;
use askama_web::WebTemplate;

pub mod users;

#[derive(Template, WebTemplate)]
#[template(path = "home_page.html")]
pub struct HomePage {
    navigation_bar: NavigationBar,
    profile: Option<User>,
}

impl HomePage {
    pub async fn get(app_state: AppState, profile: Option<User>) -> Self {
        Self {
            navigation_bar: NavigationBar::get(&app_state, &profile),
            profile,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "navigation_bar.html")]
pub struct NavigationBar {
    profile: Option<User>,
    is_admin: bool,
}

impl NavigationBar {
    pub fn get(app_state: &AppState, profile: &Option<User>) -> Self {
        let is_admin = match profile {
            Some(user) => user.is_admin(app_state),
            _ => false,
        };

        Self {
            profile: profile.clone(),
            is_admin,
        }
    }
}
