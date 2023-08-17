use serde::{Deserialize, Serialize};
use crate::make_db_id;

use crate::models::post::PostId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: CommentId,
    pub content: String,
    pub post_id: PostId,
}

make_db_id!(CommentId);

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub content: String,
    pub post_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateComment {
    pub id: CommentId,
    pub content: String,
}


#[derive(Deserialize)]
pub struct GetCommentById {
    pub comment_id: i32,
}