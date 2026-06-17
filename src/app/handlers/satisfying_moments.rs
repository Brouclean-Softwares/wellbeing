use crate::AppState;
use crate::app::handlers::DeleteForm;
use crate::app::handlers::journal::OfDayParams;
use crate::app::templates::satisfying_moments::{SatisfyingMomentPage, SatisfyingMomentsList};
use crate::data::satisfying_moments::SatisfyingMoment;
use crate::data::users::ConnectedProfile;
use crate::dates::WithTimeZone;
use crate::errors::AppError;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{Form, Router};
use chrono::NaiveDate;
use serde::Deserialize;

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route("/of_day", get(of_day))
        .route("/of_month", get(of_month))
        .route("/new", post(new))
        .route("/moment", get(moment).post(update))
        .route("/delete", post(delete))
}

async fn of_day(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<SatisfyingMomentsList, AppError> {
    let day = params.day.unwrap_or(connected_profile.today());

    let satisfying_moments =
        SatisfyingMoment::select_lived_at_for_user(&app_state, &connected_profile.user.id, &day)
            .await?;

    Ok(SatisfyingMomentsList::from(
        connected_profile,
        satisfying_moments,
        Some(day),
    ))
}

async fn of_month(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<SatisfyingMomentsList, AppError> {
    let day = params.day.unwrap_or(connected_profile.today());

    let satisfying_moments = SatisfyingMoment::select_lived_during_month_for_user(
        &app_state,
        &connected_profile.user.id,
        &day,
    )
    .await?;

    Ok(SatisfyingMomentsList::from(
        connected_profile,
        satisfying_moments,
        None,
    ))
}

#[derive(Deserialize)]
struct NewForm {
    day: NaiveDate,
    title: String,
    description: String,
}

async fn new(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Form(form): Form<NewForm>,
) -> Result<Redirect, Redirect> {
    let redirect_if_error = Redirect::to(&format!("/journal/of_day?day={}", form.day));

    let moment_id = SatisfyingMoment::insert_new(
        &app_state,
        &connected_profile.user.id,
        &form.day,
        &form.title,
        &form.description,
    )
    .await
    .or_else(|error| Err(error.log_and_redirect(redirect_if_error)))?;

    Ok(Redirect::to(&format!(
        "./moment?id={}&edit_mode=true",
        moment_id
    )))
}

#[derive(Deserialize)]
struct MomentParams {
    id: i64,
    day: Option<NaiveDate>,
    edit_mode: Option<bool>,
}

async fn moment(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<MomentParams>,
) -> Result<SatisfyingMomentPage, Redirect> {
    let redirect_if_error = match params.day {
        Some(day) => Redirect::to(&format!("/journal/of_day?day={}", day)),
        None => Redirect::to("/"),
    };

    let satisfying_moment =
        SatisfyingMoment::select_by_id_for_user(&app_state, &connected_profile.user.id, &params.id)
            .await
            .or_else(|error| Err(error.log_and_redirect(redirect_if_error)))?;

    let edit_mode = params.edit_mode.unwrap_or(false);

    Ok(SatisfyingMomentPage::from(
        connected_profile,
        satisfying_moment,
        edit_mode,
    ))
}

async fn update(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Form(moment): Form<SatisfyingMoment>,
) -> Result<Redirect, Redirect> {
    let redirect = Redirect::to(&format!("/satisfying_moments/moment?id={}", moment.id));
    let redirect_if_error = Redirect::to(&format!("/journal/of_day?day={}", moment.lived_at));

    moment
        .update(&app_state, &connected_profile.user.id)
        .await
        .or_else(|error| Err(error.log_and_redirect(redirect_if_error)))?;

    Ok(redirect)
}

async fn delete(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Form(form): Form<DeleteForm>,
) -> Result<Redirect, Redirect> {
    let redirect_if_error = Redirect::to("/");

    let moment =
        SatisfyingMoment::select_by_id_for_user(&app_state, &connected_profile.user.id, &form.id)
            .await
            .or_else(|error| Err(error.log_and_redirect(redirect_if_error)))?;

    let redirect = Redirect::to(&format!("/journal/of_day?day={}", moment.lived_at));

    moment
        .delete(&app_state, &connected_profile.user.id)
        .await
        .or_else(|error| Err(error.log_and_redirect(redirect.clone())))?;

    Ok(redirect)
}
