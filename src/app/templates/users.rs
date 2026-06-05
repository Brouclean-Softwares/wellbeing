use crate::AppState;
use crate::app::templates::NavigationBar;
use crate::data::users::Profile;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "users/user_page.html")]
pub struct UserPage {
    navigation_bar: NavigationBar,
    profile: Profile,
    is_admin: bool,
}

impl UserPage {
    pub fn from(app_state: &AppState, profile: &Profile) -> Self {
        let is_admin = match &profile.user {
            Some(user) => user.is_admin(&app_state),
            _ => false,
        };

        Self {
            navigation_bar: NavigationBar::get(&app_state, &profile),
            profile: profile.clone(),
            is_admin,
        }
    }
}
