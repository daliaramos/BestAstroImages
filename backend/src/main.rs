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
async fn main()  {
    //-> anyhow::Result<()>
    run_backend().await;

/*

    let res = ApiRes::get().await?;
    println!("{:?}", res);


    Ok(())
*/

}


