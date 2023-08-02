use serde::{Deserialize, Serialize};

use crate::question::QuestionId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

impl From<i32> for AnswerId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswer {
    pub content: String,
    pub question_id: i32,
}
