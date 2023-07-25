use std::sync::{Arc, Mutex, RwLock};
use sqlx::{PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::sync::atomic::AtomicUsize;
//use crate::answer::{Answer, AnswerId};
use crate::error::{AppError, QuestionError};
use crate::question::{Question, QuestionId};
use tracing::info;

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<Mutex<Vec<Question>>>,
  //  pub answers: Arc<RwLock<Vec<Answer>>>,
  pub conn_pool: PgPool
}

// think of Arc<>  as a shared ptr.
impl Store {
    pub async fn initialize_database_connection() -> Result<Self, AppError> {
        let db_url = std::env::var("DATABASE_URL").unwrap();

        let mut pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url).await?;

        // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool).await?;

        assert_eq!(row.0, 150);
        info!("Database test result: {}", row.0);
        let store = Store {
            questions: Arc::new(Mutex::new(vec![])),
            conn_pool: pool,
        };
        Ok(store)
    }
/*
    pub fn add_answer(&mut self, content: String, question_id:QuestionId) -> Result<Answer, AppError> {
        let mut answer = self.answers.write().unwrap();
        let len = answer.len();

        let new_answer = Answer{
            id: len,
            content,
            question_id,
        };
        answer.push(new_answer.clone());
        Ok(new_answer)
    }


 */
    pub async fn add_question(&mut self, title: String, content: String, tags: Option<Vec<String>>) -> Result<(), AppError> {
       let mut rows = sqlx::query!(
           r#"INSERT INTO "questions"(title, content, tags)
           VALUES ($1, $2, $3)
           "#,
           title,
           content,
           tags.as_deref()
       )
           .execute(&self.conn_pool).await?;
       // let new_question = Question::new(QuestionId(len as u32), title, content, tags);
       // questions.push(new_question.clone());
        Ok(())
    }

    pub fn get_all_questions(&self) -> Vec<Question> {
        self.questions.lock().unwrap().clone()
    }
    pub fn get_question_by_id(&self, id: QuestionId) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");
        let question = questions.iter().find(|q| q.id == id).cloned();
        question.ok_or(AppError::Question(QuestionError::InvalidId))
    }
    pub fn update_question(&mut self, new_question: Question) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");

        let index = new_question.id.0;

        if index as usize >= questions.len() {
            return Err(AppError::Question(QuestionError::InvalidId));
        }

        questions[index as usize] = new_question.clone();

        Ok(new_question)
    }

    pub fn delete_question(&mut self, question_id: QuestionId) -> Result<(), AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");
        questions.retain(|q | q.id != question_id);
        Ok(())
    }
}

