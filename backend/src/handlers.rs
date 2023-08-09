use argon2::Config;
use axum::extract::{Path,Query, State};
use axum::Json;
use crate::db::Store;
use crate::error::{AppError};
use crate::question::{Question, QuestionId, UpdateQuestion, CreateQuestion, GetQuestionById};
use crate::answer::{Answer, CreateAnswer};
use jsonwebtoken::Header;
use crate::get_timestamp_after_8_hours;
use crate::user::{UserSignup, Claims, User, KEYS, UpdateUser, UserCred};
use serde_json::{json, Value};
use std::fs;


pub async fn root() -> String {
    "Hello World!".to_string()
}

//CRUD - create, read, update, delete
pub async fn get_questions(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Question>>, AppError> {
    let all_questions = am_database.get_all_questions().await?;
    Ok(Json(all_questions))
}


pub async fn get_question_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<Json<Question>, AppError> {
    let question = am_database.get_question_by_id(QuestionId(query)).await?;
    Ok(Json(question))
}

pub async fn create_question(
    State(mut am_database): State<Store>,
    Json(question): Json<CreateQuestion>
) -> Result<Json<()>, AppError> {
    let new_question = am_database.add_question(question.title, question.content).await?;
    Ok(Json(new_question)) //ORM - object relational mapper
}

pub async fn update_question(
    State(mut am_database): State<Store>,
    Json(question): Json<UpdateQuestion>,
) -> Result<Json<Question>, AppError> {
    let updated_question = am_database.update_question(question).await?;
    Ok(Json(updated_question))
}


pub async fn delete_question(
    State(mut am_database): State<Store>,
    Query(query): Query<GetQuestionById>
) -> Result<(), AppError> {
    am_database.delete_question(query.question_id).await?;
    Ok(())
}
pub async fn create_answer(
    State(mut am_database): State<Store>,
    Json(answer): Json<CreateAnswer>,
) -> Result<Json<Answer>, AppError> {
    dbg!("GOT CREATE ANSWER:");
    dbg!(&answer);
    let new_answer = am_database.add_answer(answer.content, answer.question_id).await?;
    Ok(Json(new_answer))
}


//Create user account
pub async fn register(
    State(mut database) : State<Store>,
    Json(mut credentials): Json<UserSignup>
) -> Result<Json<Value>, AppError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials)
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials)
    }

    //check to see if there is already a user in the db w the given email address
    let existing_user = database.get_user(&credentials.email).await;

    if let Ok(_) = existing_user {
        return Err(AppError::UserAlreadyExists);
    }

    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    let hash_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        salt.as_bytes(),
        &hash_config
    ){
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
        }
    };
    credentials.password = hash_password;
    let new_user = database.create_user(credentials).await?;
    Ok(new_user)
}

pub async fn login(
    State(mut database): State<Store>,
    Json(creds): Json<User>
) -> Result<Json<Value>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials)
    }

    let existing_user = database.get_user(&creds.email).await?;


    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };

    if !is_password_correct {
        return Err(AppError::InvalidPassword);
    }


    if existing_user.status == "Ban".to_string() {
        return Err(AppError::AccountBanned);
    }
    //create jwt to return
    let claims = Claims {
        id: 0,
        email: creds.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;
    Ok(Json(json!({ "access_token" : token, "type": "Bearer"})))

}

pub async fn protected (
    claims: Claims,
) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area \n your claim data is: {}",
        claims
    ))
}

//g
pub async fn get_users(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<User>>, AppError> {
    let all_users = am_database.get_all_users().await?;
    Ok(Json(all_users))
}

pub async fn update_user(
    State(mut am_database): State<Store>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    let updated_user = am_database.update_user(user).await?;
    Ok(Json(updated_user))
}

pub async fn delete_user(
    State(mut am_database): State<Store>,
    Json(creds): Json<UserCred>,
) -> Result<(), AppError> {

    let existing_user = am_database.get_user(&creds.email).await?;

    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };
    if is_password_correct {
        am_database.delete_user(creds).await?
    }
    Ok(())
}
/*
pub async fn check_violation(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<Json<User>, AppError> {
    let all_questions = am_database.get_question_by_id(QuestionId(query)).await?;

    let contents = fs::read_to_string("./src/badwords.txt")
        .expect("Should have been able to read the file");

    let mut vec: Vec<String> = vec![];
    for substring in contents.split("\r\n") {
        vec.push(substring.to_string())
    }


    let message = all_questions.content.clone().to_string();

    let mut found = false;
    for words in message.split(" ") {
        if vec.contains(&words.to_string()) {
            found = true;
        }
    }

    let mut user = am_database.get_user_by_questionID(UserId(all_questions.user_id)).await?;

    if found == true{
        user.status = "Ban".to_string();
        let update_user = am_database.update_status(user).await?;
        return Ok(Json(update_user))
    }
    Ok(Json(user))
}

*/