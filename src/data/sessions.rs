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
                    (SELECT id FROM users WHERE email = $1 LIMIT 1)
                    , $2
                    , CURRENT_TIMESTAMP + interval '4 hours'
                )
                ON CONFLICT (user_id) DO UPDATE SET
                token = excluded.token,
                expires_at = CURRENT_TIMESTAMP + interval '4 hours'",
        )
        .bind(user_mail)
        .bind(session_id)
        .execute(&state.db)
        .await?;

        Ok(Session {})
    }
}
