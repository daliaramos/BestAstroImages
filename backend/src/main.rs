use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::{Body, Method, Response};
use hyper::server::conn::Http;
use hyper::service::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    println!("Hello, world!");
    let addr = SocketAddr::from(([127,0,0,1], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening...");


    loop{
    let (stream, _) = listener.accept().await.unwrap();

    tokio::task::spawn(async move {
        let http = Http::new();
        let conn = http.serve_connection(stream, service_fn(questions_handler));
        let conn = conn.await.unwrap();
    });
    }
}

async fn questions_handler(req: hyper::Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>>{
    match(req.method(), req.uri().path()){
        (&Method::GET, "/questions") => {
            println!("In Get Questions");
            Ok(Response::new("Body text".into()))
        },
        _ => {
            todo!()
        }
    }
}
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tag: Option<String>
}
impl Question {
    fn new(id: QuestionId, title:String, content: String, tag: Option<String>) -> Self {
        Question{
            id,
            title,
            content,
            tag

        }
    }
}

pub struct QuestionId(pub String);




