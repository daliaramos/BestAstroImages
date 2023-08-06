use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, title: {}, content: {}, tags: {:?}",
id,
title,
content,
tags
)]
pub struct Image {
    pub id: ImageId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}


impl Image {
    #[allow(dead_code)]
    pub fn new(id: ImageId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Image {
            id,
            title,
            content,
            tags,
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


