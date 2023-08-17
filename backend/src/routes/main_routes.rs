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
        .route("/post", get(handlers::get_posts))
        .route("/post/:post_id", get(handlers::get_post_by_id))
        .route("/post", post(handlers::create_post))
        .route("/post", put(handlers::update_post))
        .route("/post", delete(handlers::delete_post))

        .route("/comment", post(handlers::create_comment))
        .route("/comment", put(handlers::update_comment))
        .route("/comment", delete(handlers::delete_comment))

        .route("/users", post(handlers::register))
        .route("/users", put(handlers::update_user))
        .route("/users", delete(handlers::delete_user))
        .route("/login", post(handlers::login))


        //.route("/images", post(handlers::create_image))
       // .route("/images", get(handlers::create_image))
        //.route("/images", delete(handlers::delete_image))
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