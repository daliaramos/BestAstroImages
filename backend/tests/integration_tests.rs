use http::{Request, StatusCode};
use hyper::Body;
use sqlx::PgPool;
use tower::ServiceExt;

use backend::answer::CreateAnswer;
use backend::question::{CreateQuestion, Question};
use backend::routes::app;

#[sqlx::test]
async fn test_add_question(pool: PgPool){
    let mut app = app(pool).await;


    let question = CreateQuestion {
        title: "New Title".into(),
        content: "Test content2".into(),
        tags: None,
    };

    let response = app
        .unwrap()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/question")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&question).unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}