use serde_derive::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User{
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSignup{
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub struct LoggedInUser{
    pub token: Claims
}

pub struct Claims{
    pub id: i32,
    pub email: String,
    pub exp: u64
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