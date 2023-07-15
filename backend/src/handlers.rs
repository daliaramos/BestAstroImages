use axum::extract::State;
use crate::db::AppDatabase;
pub async fn root() -> String {
    "Hello World!".to_string()
}

pub async fn get_questions(
    State(am_database): State<AppDatabase>
) -> Result<Json<Vec<Question>>, AppError>{
    let mut questions = am_database.questions.lock().unwrap();

    let db_count = questions.len() as usize;
    let questions = Questions::new(
        QuestionId(db_count),
        "Default questions".to_string(),
        "Default Content".to_string(),
        Some("Default tag".to_string())
    );
    (*questions).push(questions.clone());

    let all_questions: Vec<Question> = (*questions).clone();

    Ok(Json(all_questions))
}