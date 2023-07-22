use axum::extract::{Path,Query, State};
use axum::Json;
use crate::db::Store;
use crate::error::{AppError, QuestionError};
use crate::question::{Question, QuestionId, UpdateQuestion, CreateQuestion, GetQuestionById};
use crate::answer::Answer;
pub async fn root() -> String {
    "Hello World!".to_string()
}

//CRUD - create, read, update, delete
pub async fn get_questions(
    State(am_database): State<Store>
) -> Result<Json<Vec<Question>>, AppError>{
    let all_questions = am_database.get_all_questions();
    Ok(Json(all_questions))
}

pub async fn get_questions_by_id(
    State(am_database): State<Store>,
    Path(query): Path<u32>,
) -> Result<Json<Question>, AppError> {
    let question = am_database.get_question_by_id(QuestionId(query))?;
    Ok(Json(question));
}

pub async fn create_question(
    State(mut am_database): State<Store>,
    Json(question): Json<CreateQuestion>
) -> Result<Json<Question>, AppError> {
    let new_question = am_database.add_question(question.title, question.content, question.tags)?;
    Ok(Json(new_question)) //ORM - object relational mapper
}

pub async fn update_question(
    State(mut am_database): State<Store>,
    Json(question): Json<UpdateQuestion>
) -> Result<Json<Question>, AppError> {
    let update_question = am_database.update_question(question)?;
    Ok(Json(Question))
}

pub async fn delete_question(
    State(mut am_database): State<Store>,
    Query(query): Query<GetQuestionById>
) -> Result<(), AppError> {
    am_database.delete_question(QuestionId(query.question_id))?;
    Ok(())
}

pub async fn create_answer(
    State(mut am_database): State<Store>,
    Json(answer): Json<CreateAnswer>
) -> Result<Json<Answer>, AppError> {
    let new_answer = am_database.add_answer(answer.content, answer.question_id);
    Ok(Json(new_answer))
}