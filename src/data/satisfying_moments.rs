use crate::AppState;
use crate::errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct SatisfyingMoment {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub thoughts: String,
    pub why_it_matters: String,
    pub values_alignment: String,
    pub lived_at: NaiveDate,
    pub satisfaction_level: Option<i16>,
}

impl SatisfyingMoment {
    pub async fn select_by_id_for_user(
        state: &AppState,
        user_id: &i64,
        id: &i64,
    ) -> Result<Self, AppError> {
        tracing::debug!(
            "select_by_id_for_user for id={} with user_id={}",
            id,
            user_id,
        );

        let satisfying_moment: SatisfyingMoment = sqlx::query_as(
            "SELECT id,
                        user_id,
                        title,
                        description,
                        thoughts,
                        why_it_matters,
                        values_alignment,
                        lived_at,
                        satisfaction_level
                FROM satisfying_moments
                WHERE user_id = $1
                AND id = $2",
        )
        .bind(user_id.clone())
        .bind(id.clone())
        .fetch_one(&state.db)
        .await?;

        Ok(satisfying_moment)
    }

    pub async fn select_lived_at_for_user(
        state: &AppState,
        user_id: &i64,
        lived_at: &NaiveDate,
    ) -> Result<Vec<Self>, AppError> {
        tracing::debug!(
            "select_lived_at_for_user at {} with user_id={}",
            lived_at,
            user_id,
        );

        let satisfying_moments: Vec<SatisfyingMoment> = sqlx::query_as(
            "SELECT id,
                        user_id,
                        title,
                        description,
                        thoughts,
                        why_it_matters,
                        values_alignment,
                        lived_at,
                        satisfaction_level
                FROM satisfying_moments
                WHERE user_id = $1
                AND lived_at = $2
                ORDER BY satisfaction_level DESC, lived_at ASC",
        )
        .bind(user_id.clone())
        .bind(lived_at.clone())
        .fetch_all(&state.db)
        .await?;

        Ok(satisfying_moments)
    }

    pub async fn insert_new(
        state: &AppState,
        user_id: &i64,
        lived_at: &NaiveDate,
        title: &String,
        description: &String,
    ) -> Result<i64, AppError> {
        tracing::debug!(
            "insert_new for user_id={} at {} with title={}",
            user_id,
            lived_at,
            title,
        );

        let moment_id: i64 = sqlx::query_scalar(
            "INSERT INTO satisfying_moments (
                    user_id,
                    title,
                    description,
                    lived_at)
                VALUES ($1, $2, $3, $4)
                RETURNING id",
        )
        .bind(user_id.clone())
        .bind(title.clone())
        .bind(description.clone())
        .bind(lived_at.clone())
        .fetch_one(&state.db)
        .await?;

        Ok(moment_id)
    }

    pub async fn update(&self, state: &AppState, user_id: &i64) -> Result<(), AppError> {
        tracing::debug!("update for user_id={} with id={}", user_id, self.id,);

        sqlx::query(
            "UPDATE satisfying_moments
                SET title = $3,
                    description = $4,
                    lived_at = $5,
                    thoughts = $6,
                    why_it_matters = $7,
                    values_alignment = $8,
                    satisfaction_level = $9
                WHERE user_id = $1
                AND id = $2",
        )
        .bind(user_id.clone())
        .bind(self.id.clone())
        .bind(self.title.clone())
        .bind(self.description.clone())
        .bind(self.lived_at.clone())
        .bind(self.thoughts.clone())
        .bind(self.why_it_matters.clone())
        .bind(self.values_alignment.clone())
        .bind(self.satisfaction_level.clone())
        .execute(&state.db)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, state: &AppState, user_id: &i64) -> Result<(), AppError> {
        tracing::debug!("delete for user_id={} with id={}", user_id, self.id,);

        sqlx::query(
            "DELETE
                FROM satisfying_moments
                WHERE user_id = $1
                AND id = $2",
        )
        .bind(user_id.clone())
        .bind(self.id.clone())
        .execute(&state.db)
        .await?;

        Ok(())
    }
}
