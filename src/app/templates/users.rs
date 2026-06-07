use crate::app::templates::NavigationBar;
use crate::data::users::ConnectedProfile;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "users/user_page.html")]
pub struct UserPage {
    navigation_bar: NavigationBar,
    connected_profile: ConnectedProfile,
}

impl UserPage {
    pub fn from(connected_profile: ConnectedProfile) -> Self {
        Self {
            navigation_bar: NavigationBar::from(connected_profile.clone()),
            connected_profile,
        }
    }
}
