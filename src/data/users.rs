use crate::AppState;
use crate::auth::SESSION_TOKEN;
use crate::data::sessions::Session;
use crate::errors::AppError;
use axum::extract::{FromRef, FromRequestParts};
use axum_extra::extract::PrivateCookieJar;
use http::request::Parts;
use serde::Deserialize;

#[derive(Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let MayBeUser(profile) = MayBeUser::from_request_parts(parts, state).await?;

        profile.ok_or(AppError::Unauthorized)
    }
}

pub struct MayBeUser(pub Option<User>);

impl<S> FromRequestParts<S> for MayBeUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let cookie_jar: PrivateCookieJar<AppState> =
            PrivateCookieJar::from_request_parts(parts, &state).await?;

        let token = cookie_jar.get(SESSION_TOKEN).map(|c| c.value().to_owned());

        if let Some(token) = token {
            let user = User::select_connected_user(&state, &token).await?;

            if let Some(user) = &user {
                user.extend_session_and_delete_expired(&state, &token)
                    .await?;
            }

            Ok(MayBeUser(user))
        } else {
            Ok(MayBeUser(None))
        }
    }
}

impl User {
    pub fn optional_user_eq_other(optional_user: &Option<User>, other: &Option<User>) -> bool {
        if let Some(other) = other {
            Self::optional_user_has_optional_id(optional_user, &other.id)
        } else {
            false
        }
    }

    pub fn optional_user_has_optional_id(
        optional_user: &Option<User>,
        optional_id: &Option<i64>,
    ) -> bool {
        if let (Some(user), Some(id)) = (optional_user, optional_id) {
            if let Some(user_id) = user.id {
                user_id.eq(id)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_admin(&self, state: &AppState) -> bool {
        state.admin_email.eq(&self.email)
    }

    pub async fn select_connected_user(
        state: &AppState,
        token: &String,
    ) -> Result<Option<Self>, AppError> {
        tracing::debug!("select_connected_user with token={}", token);

        let connected_user: Option<User> = sqlx::query_as(
            "SELECT users.id,
                        users.email,
                        users.name,
                        users.given_name,
                        users.family_name,
                        users.picture
                FROM sessions
                LEFT JOIN users
                ON sessions.user_id = users.id
                WHERE sessions.token = $1
                AND sessions.expires_at > CURRENT_TIMESTAMP
                LIMIT 1",
        )
        .bind(token.clone())
        .fetch_optional(&state.db)
        .await?;

        Ok(connected_user)
    }

    async fn extend_session_and_delete_expired(
        &self,
        state: &AppState,
        token: &String,
    ) -> Result<(), AppError> {
        Session::extend(state, &self.id.unwrap_or_default(), token).await?;
        Session::delete_expired(state, &self.id.unwrap_or_default(), token).await
    }

    pub async fn select_by_id(state: &AppState, id: Option<i64>) -> Result<Option<Self>, AppError> {
        tracing::debug!("select_by_id with id={}", id.unwrap_or_default());

        if let Some(user_id) = id {
            let user: Option<User> = sqlx::query_as(
                "SELECT id,
                            email,
                            name,
                            given_name,
                            family_name,
                            picture
                    FROM users
                    WHERE id = $1
                    LIMIT 1",
            )
            .bind(user_id.clone())
            .fetch_optional(&state.db)
            .await?;

            Ok(user)
        } else {
            Ok(None)
        }
    }

    pub async fn select_by_mail(state: &AppState, mail: &String) -> Result<Option<Self>, AppError> {
        tracing::debug!("select_by_mail with mail={}", mail);

        let user: Option<User> = sqlx::query_as(
            "SELECT id,
                        email,
                        name,
                        given_name,
                        family_name,
                        picture
                FROM users
                WHERE email = $1
                LIMIT 1",
        )
        .bind(mail.clone())
        .fetch_optional(&state.db)
        .await?;

        Ok(user)
    }

    pub async fn upsert(&self, state: &AppState) -> Result<Self, AppError> {
        tracing::debug!("upsert for id={}", self.id.unwrap_or_default());

        let existing_user = Self::select_by_mail(state, &self.email).await?;

        if let Some(user_id) = existing_user.and_then(|user| user.id) {
            let updated_user: User = sqlx::query_as(
                "UPDATE users
                    SET name = $2,
                        given_name = $3,
                        family_name = $4,
                        picture = $5,
                        last_updated = CURRENT_TIMESTAMP
                    WHERE id = $1
                    RETURNING users.id, users.email, users.name, given_name, family_name, users.picture",
            )
                .bind(user_id.clone())
                .bind(self.name.clone())
                .bind(self.given_name.clone())
                .bind(self.family_name.clone())
                .bind(self.picture.clone())
                .fetch_one(&state.db)
                .await?;

            Ok(updated_user)
        } else {
            let inserted_user: User = sqlx::query_as(
                "INSERT INTO users (email, name, given_name, family_name, picture)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (email) DO NOTHING
                RETURNING users.id, users.email, users.name, given_name, family_name, users.picture",
            )
                .bind(self.email.clone())
                .bind(self.name.clone())
                .bind(self.given_name.clone())
                .bind(self.family_name.clone())
                .bind(self.picture.clone())
                .fetch_one(&state.db)
                .await?;

            Ok(inserted_user)
        }
    }
}

impl PartialEq<User> for User {
    fn eq(&self, other: &User) -> bool {
        if let (Some(id), Some(other_id)) = (self.id.clone(), other.id.clone()) {
            id.eq(&other_id)
        } else {
            false
        }
    }
}

impl PartialEq<Option<User>> for User {
    fn eq(&self, other: &Option<User>) -> bool {
        if let Some(other_user) = other.clone() {
            self.eq(&other_user)
        } else {
            false
        }
    }
}
