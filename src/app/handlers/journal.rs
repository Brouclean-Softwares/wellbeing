use crate::AppState;
use crate::app::templates::journal::{
    DayNavigation, JournalOfDayPage, JournalOfMonthPage, MonthNavigation,
};
use crate::data::journal::JournalMonth;
use crate::data::users::ConnectedProfile;
use crate::dates::{DateLevel, WithTimeZone};
use crate::errors::AppError;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use chrono::NaiveDate;
use serde::Deserialize;

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route("/date_navigation", get(date_navigation))
        .route("/of_day", get(of_day))
        .route("/of_month", get(of_month))
}

#[derive(Deserialize)]
pub struct OfDayParams {
    pub day: Option<NaiveDate>,
    pub date_level: Option<DateLevel>,
}

async fn date_navigation(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<impl IntoResponse, AppError> {
    match params.date_level {
        Some(DateLevel::Month) => {
            let today = connected_profile.today();
            let month_date = params.day.unwrap_or(today);

            let journal_month =
                JournalMonth::from(&app_state, &connected_profile.user.id, month_date).await;

            Ok(MonthNavigation::from(connected_profile, journal_month).into_response())
        }

        Some(DateLevel::Day) | None => {
            Ok(DayNavigation::from(connected_profile, params.day).into_response())
        }
    }
}

async fn of_day(
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<JournalOfDayPage, AppError> {
    Ok(JournalOfDayPage::from(connected_profile, params.day))
}

async fn of_month(
    connected_profile: ConnectedProfile,
    Query(params): Query<OfDayParams>,
) -> Result<JournalOfMonthPage, AppError> {
    Ok(JournalOfMonthPage::from(connected_profile, params.day))
}
