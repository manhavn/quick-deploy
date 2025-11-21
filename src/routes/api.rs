use crate::handlers::frontend;
use axum::{
    Json, Router,
    routing::{post, put},
};
use axum_extra::extract::Multipart;
use serde_json::Value;

pub fn routes() -> Router {
    Router::new()
        .route("/server/frontend/upload", put(frontend_upload))
        .route("/sum", post(sum))
}

async fn frontend_upload(multipart: Multipart) -> Json<Value> {
    let result = frontend::upload::handler(multipart).await;
    let value = serde_json::to_value(&result).unwrap();
    Json(value)
}

#[derive(serde::Deserialize)]
struct Numbers {
    a: i32,
    b: i32,
}

#[derive(serde::Serialize)]
struct SumResponse {
    result: i32,
}

async fn sum(Json(n): Json<Numbers>) -> Json<SumResponse> {
    Json(SumResponse { result: n.a + n.b })
}
