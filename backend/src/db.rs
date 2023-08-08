use std::sync::{Arc, Mutex, RwLock};
use serde_json::Value;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use axum::Json;
use crate::answer::{Answer, AnswerId};
use crate::error::AppError;
use crate::question::{IntoQuestionId, Question, QuestionId, UpdateQuestion};
//use crate::image::{Image};
use crate::user::{User, UserSignup};
#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
    pub questions: Arc<Mutex<Vec<Question>>>,
    pub answers: Arc<RwLock<Vec<Answer>>>,
  //  pub images: Arc<Mutex<Vec<Image>>>

}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            conn_pool: pool,
            questions: Default::default(),
            answers: Default::default(),

        }
    }

    pub async fn test_database(&self) -> Result<(), sqlx::Error> {
        let row: (i64, ) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    pub async fn add_answer(
        &mut self,
        content: String,
        question_id: i32,
    ) -> Result<Answer, AppError> {
        let res = sqlx::query!(
            r#"
    INSERT INTO answers (content, question_id)
    VALUES ($1, $2)
    RETURNING *
    "#,
            content,
            question_id,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let answer = Answer {
            id: AnswerId(res.id),
            content: res.content,
            question_id: QuestionId(res.question_id.unwrap()),
        };

        Ok(answer)
    }


    pub async fn get_all_questions(&mut self) -> Result<Vec<Question>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM questions
            "#
        )
            .fetch_all(&self.conn_pool)
            .await?;

        let questions: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Question {
                    id: row.id.into(), // Assuming you have a From<u32> for QuestionId
                    title: row.title,
                    content: row.content,
                    tags: row.tags,
                }
            })
            .collect();

        Ok(questions)
    }

    pub async fn get_question_by_id<T: IntoQuestionId>(
        &mut self,
        id: T,
    ) -> Result<Question, AppError> {
        let id = id.into_question_id();

        let row = sqlx::query!(
            r#"
    SELECT * FROM questions WHERE id = $1
    "#,
            id.0,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let question = Question {
            id: row.id.into(), // Assuming you have a From<u32> for QuestionId
            title: row.title,
            content: row.content,
            tags: row.tags,
        };

        Ok(question)
    }

    pub async fn add_question(
        &mut self,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO "questions"(title, content, tags)
           VALUES ($1, $2, $3)
        "#,
            title,
            content,
            tags.as_deref()
        )
            .execute(&self.conn_pool)
            .await?;

        Ok(())
    }

    pub async fn update_question(
        &mut self,
        new_question: UpdateQuestion,
    ) -> Result<Question, AppError> {
        sqlx::query!(
            r#"
    UPDATE questions
    SET title = $1, content = $2, tags = $3
    WHERE id = $4
    "#,
            new_question.title,
            new_question.content,
            new_question.tags.as_deref(),
            new_question.id.0,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
        r#"
SELECT title, content, id, tags FROM questions WHERE id = $1
"#,
        new_question.id.0,
    )
            .fetch_one(&self.conn_pool)
            .await?;

        let question = Question {
            title: row.title,
            content: row.content,
            id: QuestionId(row.id),
            tags: row.tags,
        };

        Ok(question)
    }

    pub async fn delete_question(
        &mut self,
        question_id: i32,
    ) -> Result<(), AppError> {
        let question_id = question_id.into_question_id();
        println!("DELETE - Question id is {}", &question_id);
        sqlx::query!(
            r#"
    DELETE FROM questions WHERE id = $1
    "#,
            question_id.0,
        )
            .execute(&self.conn_pool)
            .await.unwrap();

        Ok(())
    }

    pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
      let user = sqlx::query_as::<_, User>(
          r#"
            SELECT email, password, user_role, status, question_id FROM users WHERE email = $1
          "#
      )
          .bind(email)
          .fetch_one(&self.conn_pool)
          .await?;

        Ok(user)
    }

    pub async fn get_user_by_questionID(&self, questionID: i32) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT email, password, user_role, status, question_id FROM users WHERE question_id = $1
          "#
        )
            .bind(questionID)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(user)
    }
    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        let result = sqlx::query("INSERT INTO users(email, password) values($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        }else{
            Ok(Json(serde_json::json!({"message": "User created successfully"})))
        }


    }

    pub async fn update_status(
        &mut self,
        new_status: User,
    ) -> Result<User, AppError> {
        sqlx::query!(
            r#"
                UPDATE users
                SET status = $1,
                WHERE question_id = $2
            "#,
            new_status.status,
            new_status.email,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
            r#"
                SELECT email, status FROM users WHERE question_id = $1
            "#,
            new_status.question_id,
            new_status.status
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let status = User {
            email: row.email,
            status: row.staus,
        };

        Ok(status)
    }


}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}