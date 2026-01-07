use crate::handlers::frontend::upload;
use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    routing::{post, put},
};
use axum_extra::extract::Multipart;
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new()
        .route("/server/frontend/upload", put(frontend_upload))
        .layer(DefaultBodyLimit::disable())
        .route("/sum", post(sum))
}

async fn frontend_upload(multipart: Multipart) -> Json<Value> {
    let result = match upload::handler(multipart).await {
        Ok(result) => result,
        Err(e) => return Json(json!({"error": e.to_string()})),
    };
    let value = serde_json::to_value(&result).unwrap_or(Value::Null);
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
