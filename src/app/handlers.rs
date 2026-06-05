use crate::AppState;
use crate::app::templates::HomePage;
use crate::data::users::Profile;
use axum::extract::State;

pub mod users;

pub async fn home_page(State(app_state): State<AppState>, profile: Profile) -> HomePage {
    HomePage::get(&app_state, &profile).await
}
