use axum_debug::debug_handler;
use axum::response::IntoResponse;

#[debug_handler]
async fn handler() -> impl IntoResponse {
    "hi!"
}

fn main() {}
