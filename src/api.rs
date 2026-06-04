use crate::AppState;
use axum::Router;

pub fn init_router() -> Router<AppState> {
    Router::new()
}
