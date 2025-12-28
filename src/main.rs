mod graph;

use std::env;
use std::sync::{Mutex};
use actix_cors::Cors;
use actix_web::{get, middleware, patch, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::patch;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use crate::graph::GraphBackend;

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

struct AppState {
    graph: Mutex<GraphBackend>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Starting mycelia-backend on http://localhost:8080");

    // Connect to the neo4j backend
    let uri = env::var("NEO4J_URI").expect("NEO4J_URI was not set");
    let user = env::var("NEO4J_USER").expect("NEO4J_USER was not set");
    let pass = env::var("NEO4J_PASS").expect("NEO4J_PASS was not set");
    let graph = GraphBackend::new(uri, user, pass).await;

    // Shared app state
    let app_data = web::Data::new(AppState {
        graph: Mutex::new(graph)
    });

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_origin("http://localhost:8080") // for local development
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"]);

        if let Some(cors_origin) = env::var("CORS_ORIGIN").ok() {
            cors = cors.allowed_origin(cors_origin.as_str());
        }

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(get_messages)
                    .service(patch_field)
            )
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

#[get("/messages")]
async fn get_messages(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if !check_api_key(&req) {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    let graph = data.graph.lock().expect("Failed to get mutex");
    match graph.get_messages().await {
        Ok(messages) => {
            HttpResponse::Ok().json(messages)
        }
        Err(e) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

#[derive(Deserialize, Serialize)]
struct FieldData {
    text: Option<String>
}

#[patch("/entry/{id}")]
async fn patch_field(id: web::Path<String>, body: web::Json<FieldData>, req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if !check_api_key(&req) {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing API key".to_string(),
        });
    }

    if let Some(text) = &body.text {
        let graph = data.graph.lock().expect("Failed to get mutex");
        match graph.update_text(id.as_str(), text).await {
            Ok(messages) => {
                return HttpResponse::Ok().json(messages);
            }
            Err(e) => {
                return HttpResponse::NotFound().json(ErrorResponse {
                    error: e.to_string(),
                });
            }
        }
    }

    HttpResponse::BadRequest().json(ErrorResponse {
        error: "Expected a 'text' field".to_string(),
    })
}
