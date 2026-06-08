use crate::AppState;
use crate::Language;
use crate::app::templates::NavigationBar;
use crate::data::users::{ConnectedProfile, User};
use crate::languages::Translator;
use askama::Template;
use askama_web::WebTemplate;

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
