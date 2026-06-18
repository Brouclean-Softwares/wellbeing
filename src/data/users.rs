use crate::AppState;
use crate::auth::SESSION_TOKEN;
use crate::data::sessions::Session;
use crate::dates::WithTimeZone;
use crate::errors::AppError;
use crate::locales::{Locale, Localized, Translator};
use axum::extract::{FromRef, FromRequestParts};
use axum_extra::extract::PrivateCookieJar;
use chrono::TimeZone;
use chrono_tz::Tz;
use http::request::Parts;
use serde::Deserialize;

pub struct MayBeUser(pub Option<User>);

impl<S> FromRequestParts<S> for MayBeUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let profile = Profile::from_request_parts(parts, state).await?;

        Ok(MayBeUser(profile.user))
    }
}

pub struct AdminProfile(pub ConnectedProfile);

impl<S> FromRequestParts<S> for AdminProfile
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let connected_profile = ConnectedProfile::from_request_parts(parts, state).await?;

        if connected_profile.is_admin {
            Ok(AdminProfile(connected_profile))
        } else {
            Err(AppError::Unauthorized(connected_profile.into()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedProfile {
    pub is_admin: bool,
    pub user: User,
    pub language: Locale,
    pub timezone: Tz,
}

impl Localized for ConnectedProfile {
    fn locale(&self) -> Locale {
        self.language.clone()
    }
}

impl WithTimeZone for ConnectedProfile {
    fn timezone(&self) -> impl TimeZone {
        self.timezone
    }
}

impl Translator for ConnectedProfile {}

impl TryFrom<Profile> for ConnectedProfile {
    type Error = AppError;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Some(user) = &profile.user {
            Ok(Self {
                is_admin: profile.is_admin,
                user: user.clone(),
                language: profile.language,
                timezone: profile.timezone,
            })
        } else {
            Err(AppError::Unauthorized(profile))
        }
    }
}

impl<S> FromRequestParts<S> for ConnectedProfile
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let profile = Profile::from_request_parts(parts, state).await?;

        profile.try_into()
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub is_admin: bool,
    pub user: Option<User>,
    pub language: Locale,
    pub timezone: Tz,
}

impl Localized for Profile {
    fn locale(&self) -> Locale {
        self.language.clone()
    }
}

impl WithTimeZone for Profile {
    fn timezone(&self) -> impl TimeZone {
        self.timezone
    }
}

impl Translator for Profile {}

impl From<ConnectedProfile> for Profile {
    fn from(connected_profile: ConnectedProfile) -> Self {
        Self {
            is_admin: connected_profile.is_admin,
            user: Some(connected_profile.user),
            language: connected_profile.language,
            timezone: connected_profile.timezone,
        }
    }
}

impl<S> FromRequestParts<S> for Profile
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let mut language = parts.extensions.get::<Locale>().cloned().unwrap();

        let timezone = parts
            .headers
            .get("X-Timezone")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<Tz>().ok())
            .unwrap_or(chrono_tz::UTC);

        let cookie_jar: PrivateCookieJar<AppState> =
            PrivateCookieJar::from_request_parts(parts, &state).await?;

        let token = cookie_jar.get(SESSION_TOKEN).map(|c| c.value().to_owned());

        if let Some(token) = token {
            let user = User::select_connected_user(&state, &token).await?;

            let mut is_admin = false;

            if let Some(user) = &user {
                is_admin = user.is_admin(&state);

                if let Some(preferred_language) = &user.preferred_language() {
                    language = preferred_language.clone();
                }

                user.extend_session_and_delete_expired(&state, &token)
                    .await?;
            }

            Ok(Profile {
                is_admin,
                user,
                language,
                timezone,
            })
        } else {
            Ok(Profile {
                is_admin: false,
                user: None,
                language,
                timezone,
            })
        }
    }
}

#[derive(Deserialize, Debug, sqlx::FromRow, Clone, Default)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub preferred_language: Option<String>,
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let profile = Profile::from_request_parts(parts, state).await?;

        profile.user.clone().ok_or(AppError::Unauthorized(profile))
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl User {
    pub fn is_admin(&self, state: &AppState) -> bool {
        state.admin_email.eq(&self.email)
    }

    pub fn preferred_language(&self) -> Option<Locale> {
        self.preferred_language
            .clone()
            .and_then(|language| Some(Locale::from(language)))
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
                        users.picture,
                        users.preferred_language
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
        Session::extend(state, &self.id, token).await?;
        Session::delete_expired(state, &self.id).await
    }

    pub async fn select_all(state: &AppState) -> Result<Vec<Self>, AppError> {
        tracing::debug!("select_all");

        let users: Vec<User> = sqlx::query_as(
            "SELECT id,
                        email,
                        name,
                        given_name,
                        family_name,
                        picture,
                        preferred_language
                FROM users
                ORDER BY name",
        )
        .fetch_all(&state.db)
        .await?;

        Ok(users)
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
                            picture,
                            preferred_language
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
                        picture,
                        preferred_language
                FROM users
                WHERE email = $1
                LIMIT 1",
        )
        .bind(mail.clone())
        .fetch_optional(&state.db)
        .await?;

        Ok(user)
    }

    pub async fn update(
        &self,
        state: &AppState,
        connected_profile: &ConnectedProfile,
    ) -> Result<(), AppError> {
        tracing::debug!(
            "update with user_id={} by user_id={}",
            self.id,
            connected_profile.user.id,
        );

        if !connected_profile.is_admin && connected_profile.user.ne(self) {
            return Err(AppError::Unauthorized(Profile::from(
                connected_profile.clone(),
            )));
        }

        sqlx::query(
            "UPDATE users
                SET preferred_language = $2
                WHERE id = $1",
        )
        .bind(self.id.clone())
        .bind(self.preferred_language.clone())
        .execute(&state.db)
        .await?;

        Ok(())
    }
}
