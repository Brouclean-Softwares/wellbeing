use crate::AppState;
use crate::data::users::User;
use crate::errors::AppError;
use chrono::{DateTime, Utc};

pub struct Session {
    pub user: User,
    pub last_expiration: Option<DateTime<Utc>>,
}

impl Session {
    pub async fn try_from(state: &AppState, user: User) -> Result<Self, AppError> {
        let last_expiration = Self::select_last_expiration_for_user(state, &user.id).await?;

        Ok(Self {
            user,
            last_expiration,
        })
    }

    pub async fn upsert(
        state: &AppState,
        user_mail: String,
        session_id: String,
    ) -> Result<(), AppError> {
        tracing::debug!("upsert for user_mail={}", user_mail);

        sqlx::query(
            "INSERT INTO sessions (
                    user_id,
                    token,
                    expires_at
                )
                VALUES (
                    (SELECT id FROM users WHERE email = $1 LIMIT 1),
                    $2,
                    CURRENT_TIMESTAMP + interval '48 hours'
                )",
        )
        .bind(user_mail)
        .bind(session_id)
        .execute(&state.db)
        .await?;

        Ok(())
    }

    pub async fn extend(state: &AppState, user_id: &i64, token: &String) -> Result<(), AppError> {
        tracing::debug!("extend for user_id={}", user_id);

        sqlx::query(
            "UPDATE sessions
                SET expires_at = CURRENT_TIMESTAMP + interval '48 hours'
                WHERE token = $1
                AND user_id = $2
                AND expires_at > CURRENT_TIMESTAMP",
        )
        .bind(token.clone())
        .bind(user_id.clone())
        .execute(&state.db)
        .await?;

        Ok(())
    }

    pub async fn delete_for_token(state: &AppState, token: &String) -> Result<(), AppError> {
        tracing::debug!("delete_for_token for token={}", token);

        sqlx::query(
            "DELETE
                FROM sessions
                WHERE token = $1",
        )
        .bind(token.clone())
        .execute(&state.db)
        .await?;

        Ok(())
    }

    pub async fn delete_expired(state: &AppState, user_id: &i64) -> Result<(), AppError> {
        tracing::debug!("delete_expired for user_id={}", user_id);

        sqlx::query(
            "DELETE
                FROM sessions
                WHERE user_id = $1
                AND expires_at < CURRENT_TIMESTAMP",
        )
        .bind(user_id.clone())
        .execute(&state.db)
        .await?;

        Ok(())
    }

    pub async fn select_last_expiration_for_user(
        state: &AppState,
        user_id: &i64,
    ) -> Result<Option<DateTime<Utc>>, AppError> {
        tracing::debug!("select_last_expiration_for_user for user_id={}", user_id);

        let last_expiration: Option<DateTime<Utc>> = sqlx::query_scalar(
            "SELECT MAX(expires_at)
                FROM sessions
                WHERE user_id = $1",
        )
        .bind(user_id.clone())
        .fetch_one(&state.db)
        .await?;

        Ok(last_expiration)
    }
}
