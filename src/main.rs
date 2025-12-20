mod graph;

use std::env;
use std::error::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use crate::graph::GraphBackend;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new().service(web::scope("/api").route("/test", web::get().to(manual_hello)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    let uri = env::var("NEO4J_URI").expect("NEO4J_URI was not set");
    let user = env::var("NEO4J_USER").expect("NEO4J_USER was not set");
    let pass = env::var("NEO4J_PASS").expect("NEO4J_PASS was not set");
    let graph = GraphBackend::new(uri, user, pass).await;

    match graph.test().await {
        Ok(text) => {
            HttpResponse::Ok().body(text)
        }
        Err(e) => {
            HttpResponse::NotFound().body(e.to_string())
        }
    }

}
