use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
use axum::{
    extract,
    routing::post,
    Router,
};
use reqwest::Client;

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "copyright: {},explanation: {}, hdurl: {}, media_type: {}, service_version: {}, title= {}, url={}",

copyright,
explanation,
hdurl,
media_type,
service_version,
title,
url
)]
pub struct Image {
    pub copyright: String,
   // pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String
}

pub struct QueryParams{
    pub api_key: String
}

impl Image {
    #[allow(dead_code)]
    pub fn new(image_url: String, copyright: String, explanation: String, hdurl: String, media_type: String, service_version: String, title: String, url: String) -> Self {
        Image {
            copyright,
          //  date,
            explanation,
            hdurl,
            media_type,
            service_version,
            title,
            url
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRes {
    //#[serde(flatten)]
    pub copyright: String,
  //  pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String
}



impl ApiRes {
    pub async fn get() -> Result<Self, reqwest::Error> {
        let res = reqwest::get("https://api.nasa.gov/planetary/apod?api_key=OzpcTPWl9C57laK3tZT4bz8mL87oJXW2PfDkTS5f")
            .await?
            .json::<ApiRes>()
            .await?;
        Ok(res)
    }
/*
    pub async fn post() -> Result<Self, reqwest::Error> {
        // let client = Client::new();

        let res = reqwest::get("https://api.nasa.gov/planetary/apod?api_key=OzpcTPWl9C57laK3tZT4bz8mL87oJXW2PfDkTS5f")
            .await?
            .json::<ApiRes>()
            .await?;

        Ok(res)


    }
*/

    /*
    pub async fn create_image(Json(payload): Json<ApiRes>) -> Json<ApiRes> {
        Ok(ApiRes {
             copyright: payload.copyright,
             date: payload.date,
             explanation: payload.explanation,
            hdurl: payload.hdurl,
             media_type: payload.media_type,
             service_version: payload.service_version,
             title: payload.title,
             url: payload.url
        })
    }
*/

    }






/*
impl From<PostId> for i32 {
    fn from(value: PostId) -> Self {
        value.0
    }
}


 */
#[derive(Clone, Copy, Debug, sqlx::Type, Display, derive_more::Deref, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageId(pub i32);



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImage {
    pub image_url: String,
    pub copyright: String,
  //  pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String
}