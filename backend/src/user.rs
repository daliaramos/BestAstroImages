use derive_more::Display;
use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use http::request::Parts;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;

use crate::error::AppError;
use serde_derive::{Deserialize, Serialize};
use sqlx::decode;
use crate::question::QuestionId;
/*
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, email: {}, password: {}, user_role: {}, status: {}",
id,
email,
password,
user_role,
status,
)]
*/
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User{
 //   pub id: UserId,
    pub email: String,
    pub password: String,
    pub user_role: String,
    pub status: String,
}
/*
#[derive(Clone, Copy, Debug, sqlx::Type, Display, derive_more::Deref, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub i32);

impl From<i32> for UserId {
    fn from(value: i32) -> Self {
        UserId(value)
    }
}

impl From<UserId> for i32 {
    fn from(value: UserId) -> Self {
        value.0
    }
}
 */
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSignup{
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub struct LoggedInUser{
    pub token: Claims
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: String,
    pub status: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCred {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, derive_more::Display)]
#[display(fmt = "id: {}, email: {}, exp: {}", id, email, exp)]
pub struct Claims{
    pub id: i32,
    pub email: String,
    pub exp: u64
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        //extract a token claims from our Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub struct Keys{
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET")
        .expect("MISSING JWT SECRET");

    Keys::new(secret.as_bytes())
});