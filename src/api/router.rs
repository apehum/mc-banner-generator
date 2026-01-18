use crate::api::state::AppState;
use crate::api::v1::generate::generate;
use crate::api::v1::meta::meta;
use crate::api::v1::pattern::pattern;
use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/generate/{banner}", get(generate))
        .route("/api/v1/meta/{banner}", get(meta))
        .route("/api/v1/pattern/{pattern}", get(pattern))
        .fallback_service(ServeDir::new("web/dist"))
        .layer(
            TraceLayer::new_for_http()
                .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Nanos)),
        )
        .layer(
            CorsLayer::new().allow_origin(Any)
        )
}
