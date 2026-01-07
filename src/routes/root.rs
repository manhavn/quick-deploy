use axum::{Router, routing::get};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn routes() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            let now = SystemTime::now();
            let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
            format!("Rust Server Time {}", since_epoch.as_secs())
        }),
    )
}
