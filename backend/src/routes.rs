use axum::response::Response;
use axum::Router;
use axum::routing::*;
use http::StatusCode;
use hyper::Body;
use axum::routing::get;
use crate::db::Store;
use crate::handlers;
pub fn get_router() -> Router {
    let db = Store::default();
    Router::new()
        .route("/questions", get(handlers::get_questions))
        .route("/question/:question_id", get(handlers::get_question_by_id))
        .route("/question", post(handlers::create_question))
        .route("/question", put(handlers::update_question))
        .route("/question", delete(handlers::delete_question))

        .route("/answer", post(handlers::create_answer))
        .route("/*_", get(handle_404))
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The request page could not be found"))
        .unwrap()
}