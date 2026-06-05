use crate::AppState;
use crate::errors::AppError;

pub struct Session {}

impl Session {
    pub async fn upsert(
        state: &AppState,
        user_mail: String,
        session_id: String,
    ) -> Result<Self, AppError> {
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
                    CURRENT_TIMESTAMP + interval '4 hours'
                )",
        )
        .bind(user_mail)
        .bind(session_id)
        .execute(&state.db)
        .await?;

        Ok(Session {})
    }

    pub async fn extend(state: &AppState, user_id: &i64, token: &String) -> Result<(), AppError> {
        tracing::debug!("extend for user_id={}", user_id);

        sqlx::query(
            "UPDATE sessions
                SET expires_at = CURRENT_TIMESTAMP + interval '4 hours'
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
}
