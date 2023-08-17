use argon2::Config;
use axum::{Form, Json};
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use crate::db::Store;
use crate::error::{AppError};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;

use tracing::error;
use crate::get_timestamp_after_8_hours;

use serde_json::{json, Value};
use tera::Context;

use std::fs;

use crate::models::user::{UserSignup, Claims, User, KEYS, UpdateUser, UserCred, OptionalClaims};
use crate::models::post::{Post, PostId, UpdatePost, CreatePost, GetPostById};
use crate::models::comment::{Comment, CreateComment, UpdateComment, GetCommentById};
use crate::models::image::{Image, CreateImage, ApiRes, UpdateImage, GetImageById};
use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Dalia");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the page data
    //    let page_packages = am_database.get_all_question_pages().await?;
     //   context.insert("page_packages", &page_packages);

       "images.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

//CRUD - create, read, update, delete
pub async fn get_posts(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Post>>, AppError> {
    let all_posts = am_database.get_all_posts().await?;
    Ok(Json(all_posts))
}


pub async fn get_post_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<Json<Post>, AppError> {
    let post = am_database.get_post_by_id(PostId(query)).await?;
    Ok(Json(post))
}

pub async fn create_post(
    State(mut am_database): State<Store>,
    Json(post): Json<CreatePost>
) -> Result<Json<()>, AppError> {
    let new_post= am_database.add_post(post.title, post.content).await?;
    Ok(Json(new_post)) //ORM - object relational mapper
}

pub async fn update_post(
    State(mut am_database): State<Store>,
    Json(post): Json<UpdatePost>,
) -> Result<Json<Post>, AppError> {
    let updated_post = am_database.update_post(post).await?;
    Ok(Json(updated_post))
}

pub async fn delete_post(
    State(mut am_database): State<Store>,
    Query(query): Query<GetPostById>
) -> Result<(), AppError> {
    am_database.delete_post(query.post_id).await?;
    Ok(())
}
pub async fn create_comment(
    State(mut am_database): State<Store>,
    Json(comment): Json<CreateComment>,
) -> Result<Json<Comment>, AppError> {
    let new_comment = am_database.add_comments(comment.content, comment.post_id).await?;
    Ok(Json(new_comment))
}

pub async fn update_comment(
    State(mut am_database): State<Store>,
    Json(comment): Json<UpdateComment>,
) -> Result<Json<Comment>, AppError> {
    let new_comment = am_database.update_comment(comment).await?;
    Ok(Json(new_comment))
}

pub async fn delete_comment(
    State(mut am_database): State<Store>,
    Query(query): Query<GetCommentById>
) -> Result<(), AppError> {
    am_database.delete_comment(query.comment_id).await?;
    Ok(())
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
    Form(creds): Form<UserCred>
) -> Result<Response<Body>, AppError> {
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
    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)

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
pub async fn create_image(
    State(mut am_database): State<Store>,
   // Json(payload): Json<CreateImage>
) -> Result<Json<Image>, AppError> {
    let new_image= am_database.get_image().await?;
    Ok(Json(new_image)) //ORM - object relational mapper
}

pub async fn delete_image(
    State(mut am_database): State<Store>,
    Query(query): Query<GetImageById>
) -> Result<(), AppError> {
    am_database.delete_image(query.image_id).await?;
    Ok(())
}

pub async fn get_image(
    State(mut am_database): State<Store>,
) -> Result<Json<ApiRes>, reqwest::Error> {
    let new_image = am_database.get_image().await?;
    Ok(Json(new_image))
}




pub async fn create_image(
    State(mut am_database): State<Store>,
    extract::Json(payload): extract::Json<CreateImage>) {
    // payload is a `CreateUser`
}


 */
/*
pub async fn check_violation(
     State(mut am_database): State<Store>,
    Json(user): Json<UpdateUser>,
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