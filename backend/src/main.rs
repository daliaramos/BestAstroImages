use backend::run_backend;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
//use serde::Deserialize;
use http::{Request, Response, StatusCode};
use backend::models::image::ApiRes;
use axum::{
    Json,
    routing::post,
    Router,
};
use serde_json::Value;
#[tokio::main]
async fn main() {
    //-> anyhow::Result<()>
    run_backend().await;



    //let res = ApiRes::get().await?;
    //println!("{:?}", res);


    //Ok(())


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
