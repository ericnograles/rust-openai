use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Debug, Deserialize, Serialize)]
struct IngestData {
    organization_id: Uuid,
    metadata: serde_json::Value,
}

#[post("/api/v1/ingest")]
async fn ingest(data: web::Json<IngestData>) -> impl Responder {
    let ingest_data = data.into_inner();
    println!("Received data: {:?}", ingest_data);

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Data ingested successfully"
    }))
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
