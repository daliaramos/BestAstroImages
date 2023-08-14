use serde_derive::{Deserialize, Serialize};
use crate::make_db_id;
use axum::Json;
use axum::response::{IntoResponse, Response};
/*
// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, copyright: {},explanation: {}, hdurl: {}, media_type: {}, service_version: {}, title= {}, url={}",
id,
copyright,
explanation,
hdurl,
media_type,
service_version,
title,
url
)]

 */

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Image {
    pub id: Option<ImageId>,
    pub copyright: String,
   // pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String
}


make_db_id!(ImageId);

impl IntoResponse for Image{
    fn into_response(self) -> Response {
        Json(self).into_response()
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



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImage {
    pub copyright: String,
  //  pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateImage {
    pub copyright: String,
    //  pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String
}

#[derive(Deserialize)]
pub struct GetImageById {
    pub image_id: i32,
}