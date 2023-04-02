use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;
mod auth;
mod kafka_producer;

use auth::validate_jwt;
use kafka_producer::send_to_kafka;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IngestData {
    organization_id: Uuid,
    metadata: serde_json::Value,
}

#[post("/api/v1/ingest")]
async fn ingest(request: HttpRequest, data: web::Json<IngestData>) -> impl Responder {
    if let Err(_) = validate_jwt(&request) {
        return HttpResponse::Unauthorized().body("Invalid JWT");
    }

    let ingest_data = data.into_inner();
    println!("Received data: {:?}", ingest_data);

    let send_result = send_to_kafka(ingest_data).await;

    match send_result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Data ingested successfully"
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to send data to Kafka: {}", err)
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(ingest)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
