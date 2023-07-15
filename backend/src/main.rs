mod db;
mod routes;
mod error;
mod layers;

use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use axum::Router;
use axum::routing::{get, MethodRouter};
use tokio::net::TcpListener;
use hyper::{Body, Method, Response};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use dotenvy::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    //grabs everthing in the .env file and makes it available.
    dotenv().ok();
    init_logging();

    let addr = get_host_from_end();

    let (cors_layers, trace_layers) = layers::get_layers();
   // let app = Router::new()
   //     .route("/questions", get(hello_world));

    let app = routes::get_router().layer(cors_layers).layer(trace_layers);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> String {
    "Hello World!".to_string()
}
fn get_host_from_end() -> SocketAddr {
    let host = std::env::var("API_HOST").unwrap();
    let api_host = IpAddr::from_str(&host).unwrap();
    let api_port: u16 = std::env::var("API_PORT")
        .unwrap()
        .parse()
        .unwrap();

    SocketAddr::from((api_host, api_port))
}
fn init_logging() {
    // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "backend=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}



