use crate::AppState;
use crate::app::templates::NavigationBar;
use crate::data::users::ConnectedProfile;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "users/user_page.html")]
pub struct UserPage {
    navigation_bar: NavigationBar,
    connected_profile: ConnectedProfile,
    is_admin: bool,
}

impl UserPage {
    pub fn get(app_state: &AppState, connected_profile: &ConnectedProfile) -> Self {
        let is_admin = connected_profile.user.is_admin(&app_state);

        Self {
            navigation_bar: NavigationBar::get_from_connected_profile(
                &app_state,
                &connected_profile,
            ),
            connected_profile: connected_profile.clone(),
            is_admin,
        }
    }
}
