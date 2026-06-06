use crate::AppState;
use crate::app::templates::satisfying_moments::SatisfyingMomentsOfTheDay;
use crate::data::satisfying_moments::SatisfyingMoment;
use crate::data::users::ConnectedProfile;
use crate::dates::WithTimeZone;
use crate::errors::AppError;
use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use chrono::NaiveDate;
use serde::Deserialize;

pub fn init_router() -> Router<AppState> {
    Router::new().route("/of_the_day", get(of_the_day))
}

#[derive(Deserialize)]
pub struct OfTheDayParams {
    pub day: Option<NaiveDate>,
}

pub async fn of_the_day(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<OfTheDayParams>,
) -> Result<SatisfyingMomentsOfTheDay, AppError> {
    let day = params.day.unwrap_or(connected_profile.today());

    let satisfying_moments =
        SatisfyingMoment::select_lived_at_for_user(&app_state, &day, &connected_profile.user.id)
            .await?;

    Ok(SatisfyingMomentsOfTheDay::get(
        &connected_profile,
        &day,
        &satisfying_moments,
    ))
}
