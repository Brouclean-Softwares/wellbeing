use crate::AppState;
use crate::app::templates::users::UserPage;
use crate::data::users::ConnectedProfile;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

pub fn init_router() -> Router<AppState> {
    Router::new().route("/user", get(user))
}

pub async fn user(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
) -> UserPage {
    UserPage::get(&app_state, &connected_profile)
}
