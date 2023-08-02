use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::db::new_pool;

pub mod answer;
pub mod db;
pub mod error;
pub mod handlers;
pub mod layers;
pub mod question;
pub mod routes;

pub async fn run_backend() {
    dotenv().ok();
    init_logging();

    let addr = get_host_from_env();

    let app = routes::app(new_pool().await).await;

    info!("Listening...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_host_from_env() -> SocketAddr {
    let host = std::env::var("API_HOST").unwrap();
    let api_host = IpAddr::from_str(&host).unwrap();
    let api_port: u16 = std::env::var("API_PORT")
        .expect("Could not find API_PORT in .env file")
        .parse()
        .expect("Can't create a u16 from the given API_PORT string");

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
