use crate::AppState;
use crate::app::templates::users::{UserPage, UsersPage};
use crate::data::sessions::Session;
use crate::data::users::{AdminProfile, ConnectedProfile, User};
use crate::errors::AppError;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use axum::{Form, Router};
use serde::Deserialize;

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route("/", get(users))
        .route("/user", get(user).post(update))
}

async fn users(
    State(app_state): State<AppState>,
    AdminProfile(connected_profile): AdminProfile,
) -> Result<UsersPage, AppError> {
    let users = User::select_all(&app_state).await?;

    let mut sessions: Vec<(Session, i64)> = Vec::with_capacity(users.len());

    for user in users {
        let last_month_entries_count = user.select_last_month_entries_count(&app_state).await?;
        let session = Session::try_from(&app_state, user).await?;

        sessions.push((session, last_month_entries_count));
    }

    Ok(UsersPage::from(connected_profile, sessions))
}

#[derive(Deserialize)]
struct UserParams {
    id: Option<i64>,
}

async fn user(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<UserParams>,
) -> Result<UserPage, AppError> {
    let id = params.id.unwrap_or(connected_profile.user.id);

    match User::select_by_id(&app_state, Some(id)).await? {
        Some(user) => {
            if user == connected_profile.user || connected_profile.is_admin {
                Ok(UserPage::from(app_state, connected_profile, user))
            } else {
                Err(AppError::Unauthorized(connected_profile.into()))
            }
        }

        None => Err(AppError::Unauthorized(connected_profile.into())),
    }
}

#[derive(Deserialize)]
struct UpdateForm {
    id: i64,
    preferred_language: Option<String>,
}

async fn update(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Form(form): Form<UpdateForm>,
) -> Result<Redirect, Redirect> {
    let redirect = Redirect::to(&format!("/users/user?id={}", form.id));

    let user = User::select_by_id(&app_state, Some(form.id))
        .await
        .or_else(|error| Err(error.log_and_redirect(redirect.clone())))?;

    if let Some(mut user) = user {
        user.preferred_language = form.preferred_language;

        user.update(&app_state, &connected_profile)
            .await
            .or_else(|error| Err(error.log_and_redirect(redirect.clone())))?;
    }

    Ok(redirect)
}
