use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
//use crate::user::UserId;
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

  //  pub user_id: UserId,
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

#[derive(Clone, Copy, Debug, sqlx::Type, Display, derive_more::Deref, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PostId(pub i32);

impl From<i32> for PostId {
    fn from(value: i32) -> Self {
        PostId(value)
    }
}

impl From<PostId> for i32 {
    fn from(value: PostId) -> Self {
        value.0
    }
}


pub trait IntoPostId {
    fn into_question_id(self) -> PostId;
}

impl IntoPostId for i32 {
    fn into_question_id(self) -> PostId {
        PostId::from(self)
    }
}

impl IntoPostId for PostId {
    fn into_question_id(self) -> PostId {
        self
    }
}

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
  //  pub user_id: UserId
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
    pub tags: Option<Vec<String>>,
}

