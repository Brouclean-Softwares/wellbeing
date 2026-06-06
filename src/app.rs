use crate::AppState;
use axum::Router;
use axum::routing::get;

pub mod handlers;
pub mod templates;

pub fn init_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/satisfying_moments",
            handlers::satisfying_moments::init_router(),
        )
        .nest("/users", handlers::users::init_router())
        .route("/", get(handlers::home_page))
}
