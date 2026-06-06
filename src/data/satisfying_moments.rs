use crate::AppState;
use crate::errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct SatisfyingMoment {
    pub id: i64,
    pub user_id: i64,
    pub description: String,
    pub thoughts: Option<String>,
    pub why_it_matters: Option<String>,
    pub values_alignment: Option<String>,
    pub lived_at: NaiveDate,
    pub satisfaction_level: Option<i16>,
}

impl SatisfyingMoment {
    pub async fn select_lived_at_for_user(
        state: &AppState,
        lived_at: &NaiveDate,
        user_id: &i64,
    ) -> Result<Vec<Self>, AppError> {
        tracing::debug!(
            "select_lived_at_for_user at {} with user_id={}",
            lived_at,
            user_id
        );

        let satisfying_moments: Vec<SatisfyingMoment> = sqlx::query_as(
            "SELECT id,
                        user_id,
                        description,
                        thoughts,
                        why_it_matters,
                        values_alignment,
                        lived_at,
                        satisfaction_level
                FROM satisfying_moments
                WHERE lived_at = $1
                AND user_id = $2",
        )
        .bind(lived_at.clone())
        .bind(user_id.clone())
        .fetch_all(&state.db)
        .await?;

        Ok(satisfying_moments)
    }
}
