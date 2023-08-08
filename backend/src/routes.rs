use axum::response::Response;
use axum::Router;
use axum::routing::*;
use http::StatusCode;
use hyper::Body;
use tracing::info;

use crate::{handlers, layers};
use crate::db::Store;
use crate::handlers::root;
use sqlx::PgPool;

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    info!("Seeded database");

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .route("/", get(root))
        .route("/questions", get(handlers::get_questions))
        .route("/question/:question_id", get(handlers::get_question_by_id))
        .route("/question", post(handlers::create_question))
        .route("/question", put(handlers::update_question))
        .route("/question", delete(handlers::delete_question))

        .route("/answer", post(handlers::create_answer))
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        //.route("/users?email=email", get(handlers::get_user))
        //.route("/image", post(handlers::create_image))
        .route("/report/:question_id", get(handlers::check_violation))
        .route("/protected", get(handlers::protected))
        .route("/*_", get(handle_404))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}