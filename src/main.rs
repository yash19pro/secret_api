use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use base64::{Engine as _, engine::general_purpose};

use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    mode: String,
    message: String,
}

#[post("/transform")]
async fn transform(data: web::Json<Request>) -> impl Responder {
    match data.mode.as_str() {
        "encode" => {
            let encoded = general_purpose::STANDARD.encode(&data.message);
            HttpResponse::Ok().json(serde_json::json!({ "result": encoded }))
        }
        "decode" => match general_purpose::STANDARD.decode(&data.message) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(decoded_str) => {
                    HttpResponse::Ok().json(serde_json::json!({ "result": decoded_str }))
                }
                Err(_) => HttpResponse::BadRequest().body("Invalid UTF-8 in decoded string"),
            },
            Err(_) => HttpResponse::BadRequest().body("Invalid base64 input"),
        },
        _ => HttpResponse::BadRequest().body("Invalid mode. Use 'encode' or 'decode'"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8080");

    HttpServer::new(|| App::new().service(transform))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
