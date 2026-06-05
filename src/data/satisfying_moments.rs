use crate::AppState;
use crate::errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct SatisfyingMoments {
    pub id: i64,
    pub user_id: i64,
    pub description: String,
    pub thoughts: Option<String>,
    pub why_it_matters: Option<String>,
    pub values_alignment: Option<String>,
    pub lived_at: NaiveDate,
    pub satisfaction_level: Option<i16>,
}

impl SatisfyingMoments {
    pub async fn select_all_for_user(
        state: &AppState,
        user_id: &i64,
    ) -> Result<Vec<Self>, AppError> {
        tracing::debug!("select_all_for_user with user_id={}", user_id);

        let satisfying_moments: Vec<SatisfyingMoments> = sqlx::query_as(
            "SELECT id,
                        user_id,
                        description,
                        thoughts,
                        why_it_matters,
                        values_alignment,
                        lived_at,
                        satisfaction_level
                FROM satisfying_moments
                WHERE user_id = $1",
        )
        .bind(user_id.clone())
        .fetch_all(&state.db)
        .await?;

        Ok(satisfying_moments)
    }
}
