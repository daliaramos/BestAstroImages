use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, image_url: {}",
id,
image_url
)]
pub struct Image {
    pub id: ImageId,
    pub image_url: String,
}


impl Image {
    #[allow(dead_code)]
    pub fn new(id: ImageId, image_url: String) -> Self {
        Image {
            id,
            image_url
        }
    }
}

#[derive(Clone, Copy, Debug, sqlx::Type, Display, derive_more::Deref, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageId(pub i32);

impl From<i32> for ImageId {
    fn from(value: i32) -> Self {
        ImageId(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImage {
    pub image_url: String,
}