pub mod analytics;
pub mod booking;
pub mod energy;
pub mod plan;
use sqlx::PgPool;
use axum::{
    Router,
    routing::{get, post},
};
use crate::AppState;
use std::sync::Arc;
mod super_handlers {
    // re-export handlers from crate root handlers module
    pub use crate::handlers::{analytics, charging_plan, energy_status, ev_request, ev_requests};
}
pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ev-request", post(super_handlers::ev_request))
        .route("/ev-requests", get(super_handlers::ev_requests))
        .route("/energy-status", get(super_handlers::energy_status))
        .route("/charging-plan", get(super_handlers::charging_plan))
        .route("/analytics", get(super_handlers::analytics))
        .with_state(state)
}
