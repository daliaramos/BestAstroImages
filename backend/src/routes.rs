use axum::response::Response;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use axum::routing::get;
use crate::db::AppDatabase;
pub fn get_router() -> Router {
    let db = AppDatabase::default();
    Routes::new()
        .route("/questions", get(handlers::get_questions))
        .route("/*_", get(handle_404))
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The request page could not be found"))
        .unwrap()
}