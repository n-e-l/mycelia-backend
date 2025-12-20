mod graph;

use std::env;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Serialize;
use crate::graph::GraphBackend;

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new().service(web::scope("/api").route("/test", web::get().to(manual_hello)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn check_api_key(req: &HttpRequest) -> bool {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.strip_prefix("Bearer ").unwrap_or(h))
        .map(|key| key == env::var("API_KEY").expect("API_KEY was not set"))
        .unwrap_or(false)
}

async fn manual_hello(req: HttpRequest) -> impl Responder {
    if !check_api_key(&req) {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let uri = env::var("NEO4J_URI").expect("NEO4J_URI was not set");
    let user = env::var("NEO4J_USER").expect("NEO4J_USER was not set");
    let pass = env::var("NEO4J_PASS").expect("NEO4J_PASS was not set");
    let graph = GraphBackend::new(uri, user, pass).await;

    match graph.test().await {
        Ok(text) => {
            HttpResponse::Ok().json(ApiResponse {
                message: text,
            })
        }
        Err(e) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }

}
