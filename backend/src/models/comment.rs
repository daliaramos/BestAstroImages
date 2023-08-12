use serde::{Deserialize, Serialize};

use crate::models::post::PostId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: CommentId,
    pub content: String,
    pub post_id: PostId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentId(pub i32);

impl From<i32> for CommentId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub content: String,
    pub post_id: i32,
}
