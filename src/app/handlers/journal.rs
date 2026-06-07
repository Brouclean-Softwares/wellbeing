use crate::AppState;
use crate::app::templates::journal::{DayNavigation, JournalOfDayPage};
use crate::data::users::ConnectedProfile;
use crate::errors::AppError;
use axum::Router;
use axum::extract::Query;
use axum::routing::get;
use chrono::NaiveDate;
use serde::Deserialize;

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route("/of_day", get(of_day))
        .route("/day_navigation", get(day_navigation))
}

#[derive(Deserialize)]
pub struct OfDayParams {
    pub day: Option<NaiveDate>,
}

async fn of_day(
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<JournalOfDayPage, AppError> {
    Ok(JournalOfDayPage::from(connected_profile, params.day))
}

async fn day_navigation(
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<DayNavigation, AppError> {
    Ok(DayNavigation::from(connected_profile, params.day))
}
