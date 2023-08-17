use crate::make_db_id;
use serde_derive::{Deserialize, Serialize};
// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, title: {}, content: {}",
id,
title,
content,
)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
}


impl Post {
    #[allow(dead_code)]
    pub fn new(id: PostId, title: String, content: String) -> Self {
        Post {
            id,
            title,
            content,
        }
    }
}

make_db_id!(PostId);


// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetPostById {
    pub post_id: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: PostId,
    pub title: String,
    pub content: String,
}

