use std::sync::{Arc, Mutex, RwLock};
use serde_json::Value;
use sqlx::PgPool;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use axum::{Json, extract};
use crate::models::comment::{Comment, CommentId};
use crate::error::AppError;
use crate::models::post::{Post, PostId, UpdatePost, CreatePost, GetPostById, IntoPostId};
use crate::models::image::{Image, CreateImage, ImageId, ApiRes, QueryParams};
use crate::models::user::{User, UserSignup, UpdateUser, UserCred};
#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
    pub posts: Arc<Mutex<Vec<Post>>>,
    pub comments: Arc<RwLock<Vec<Comment>>>,
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
            posts: Default::default(),
            comments: Default::default(),

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

    pub async fn add_comments(
        &mut self,
        content: String,
        post_id: i32,
    ) -> Result<Comment, AppError> {
        let res = sqlx::query!(
            r#"
    INSERT INTO comments (content, post_id)
    VALUES ($1, $2)
    RETURNING *
    "#,
            content,
            post_id,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let comment = Comment {
            id: CommentId(res.id),
            content: res.content,
            post_id: PostId(res.post_id.unwrap()),
        };

        Ok(comment)
    }


    pub async fn get_all_posts(&mut self) -> Result<Vec<Post>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM posts
            "#
        )
            .fetch_all(&self.conn_pool)
            .await?;

        let posts: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Post {
                    id: row.id.into(), // Assuming you have a From<u32> for QuestionId
                    title: row.title,
                    content: row.content,
                    //user_id: row.user_id
                }
            })
            .collect();

        Ok(posts)
    }

    pub async fn get_post_by_id<T: IntoPostId>(
        &mut self,
        id: T,
    ) -> Result<Post, AppError> {
        let id = id.into_question_id();

        let row = sqlx::query!(
            r#"
    SELECT * FROM posts WHERE id = $1
    "#,
            id.0,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let post = Post {
            id: row.id.into(), // Assuming you have a From<u32> for QuestionId
            title: row.title,
            content: row.content,
           // user_id: UserId(row.user_id.unwrap())
        };

        Ok(post)
    }

    pub async fn add_post(
        &mut self,
        title: String,
        content: String,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO "posts"(title, content)
           VALUES ($1, $2)
        "#,
            title,
            content,
        )
            .execute(&self.conn_pool)
            .await?;

        Ok(())
    }

    pub async fn update_post(
        &mut self,
        new_post: UpdatePost,
    ) -> Result<Post, AppError> {
        sqlx::query!(
            r#"
    UPDATE posts
    SET title = $1, content = $2
    WHERE id = $3
    "#,
            new_post.title,
            new_post.content,
            new_post.id.0,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
        r#"
            SELECT title, content, id FROM posts WHERE id = $1
        "#,
        new_post.id.0,
    )
            .fetch_one(&self.conn_pool)
            .await?;

        let post = Post {
            title: row.title,
            content: row.content,
            id: PostId(row.id),
            //user_id: UserId(row.user_id.unwrap())
        };

        Ok(post)
    }

    pub async fn delete_post(
        &mut self,
        post_id: i32,
    ) -> Result<(), AppError> {
        let post_id = post_id.into_question_id();
        println!("DELETE - Question id is {}", &post_id);
        sqlx::query!(
            r#"
                DELETE FROM posts WHERE id = $1
            "#,
            post_id.0,
        )
            .execute(&self.conn_pool)
            .await.unwrap();

        Ok(())
    }

    pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
      let user = sqlx::query_as::<_, User>(
          r#"
            SELECT email, password, user_role, status FROM users WHERE email = $1
          "#
      )
          .bind(email)
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

    pub async fn get_all_users(&mut self) -> Result<Vec<User>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM users
            "#
        )
            .fetch_all(&self.conn_pool)
            .await?;

        let user: Vec<_> = rows
            .into_iter()
            .map(|row| {
                User {
                 //   id: row.id.into(), // Assuming you have a From<u32> for QuestionId
                    email: row.email,
                    password: row.password,
                    user_role: row.user_role,
                    status: row.status
                }
            })
            .collect();

        Ok(user)
    }

    pub async fn update_user(
        &mut self,
        new_user: UpdateUser,
    ) -> Result<User, AppError> {
        sqlx::query!(
            r#"
                UPDATE users
                SET status = $1
                WHERE email = $2
            "#,
            new_user.status,
            new_user.email
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
        r#"
            SELECT email, password, user_role, status FROM users WHERE email = $1
        "#,
        new_user.email,
    )
            .fetch_one(&self.conn_pool)
            .await?;

        let user = User {
            email: row.email,
            password: row.password,
            user_role: row.user_role,
            status: row.status,
            //user_id: UserId(row.user_id.unwrap())
        };

        Ok(user)
    }

    pub async fn delete_user(
        &mut self,
        user_cred: UserCred,
    ) -> Result<(), AppError> {

        sqlx::query!(
            r#"
                DELETE FROM users WHERE email = $1
            "#,
           user_cred.email,
        )
            .execute(&self.conn_pool)
            .await.unwrap();

        Ok(())
    }

    pub async fn add_image(
        &mut self,
        payload: CreateImage,
    ) -> Result<Image, AppError> {

        //if not in the db then we want to call nasa api

        let res = sqlx::query!(
            r#"
                INSERT INTO images (copyright, explanation,hdurl, media_type, service_version, title, url)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#,
            payload.copyright,
            payload.explanation,
            payload.hdurl,
            payload.media_type,
            payload.service_version,
            payload.title,
            payload.url
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let image = Image {
            copyright: res.copyright,
            explanation: res.explanation,
            hdurl: res.hdurl,
            media_type: res.media_type,
            service_version: res.service_version,
            title: res.title,
            url: res.url
        };

        Ok(image)
    }


    pub async fn get_image(
        &mut self,
    ) -> Result<ApiRes, reqwest::Error> {
        let res = ApiRes::get().await?;
        Ok(res)
    }




/*
    pub async fn get_user_by_questionID(&self, questionID: UserId) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password, user_role, status FROM users WHERE id = $1
          "#
        )
            .bind(1)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(user)
    }
    pub async fn update_status(
        &mut self,
        new_status: User,
    ) -> Result<User, AppError> {
        sqlx::query!(
            r#"
                UPDATE users
                SET status = $1
                WHERE id = $2
            "#,
            new_status.status,
            *new_status.id,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
            r#"
                SELECT id, email, password, user_role, status FROM users WHERE email = $1
            "#,
            new_status.email,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let status = User {
            id: UserId(row.id),
          email:  row.email,
            password: row.password,
            user_role: row.user_role,
            status: row.status
        };

        Ok(status)
    }

    pub async fn get_user_by_questionID(&self, questionID: UserId) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password, user_role, status FROM users WHERE id = $1
          "#
        )
            .bind(1)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(user)
    }
 */
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}