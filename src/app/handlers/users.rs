use crate::AppState;
use crate::app::templates::users::UserPage;
use crate::data::users::ConnectedProfile;
use axum::Router;
use axum::routing::get;

pub fn init_router() -> Router<AppState> {
    Router::new().route("/user", get(user))
}

async fn user(connected_profile: ConnectedProfile) -> UserPage {
    UserPage::from(connected_profile)
}
