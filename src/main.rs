use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

// Define a struct para a resposta JSON. O atributo `#[derive(Serialize)]` permite que a struct seja convertida para JSON
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

// Define o handler para a rota "/health". A macro `#[get("/health")]` faz o roteamento
#[get("/health")]
async fn health_check() -> impl Responder {
    // Cria a resposta e a converte para JSON, retornando um status 200 OK
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

// A função principal que inicia o servidor. A macro `#[actix_web::main]` prepara o ambiente assíncrono
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8082;
    println!("🚀 Servidor iniciado na porta {}", port);

    HttpServer::new(|| {
        // Registra os handlers na aplicação
        App::new().service(health_check)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}