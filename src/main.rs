use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs; // Para manipulação de arquivos

// Estruturas de Dados

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
}

#[derive(Serialize)]
struct ScrapeResponse {
    preco: f64,
}

// Lógica de Extração de Preço

fn extract_price_from_html(html: &str) -> Option<f64> {
    lazy_static! {
        static ref PRICE_PATTERNS: Vec<Regex> = vec![
            // Padrão 1: Para <span class="a-price-whole">299<span class="a-price-decimal">,</span></span><span class="a-price-fraction">00</span>
            Regex::new(r#"<span class="a-price-whole">([\d\.,]+)<span class="a-price-decimal">,</span></span><span class="a-price-fraction">(\d{2})"#).unwrap(),
            // Padrão 2: Para <span class="a-offscreen">R$299,00</span>
            Regex::new(r#"<span class="a-offscreen">R\$\s?([\d\.,]+)</span>"#).unwrap(),
            // Padrões antigos mantidos para compatibilidade
            Regex::new(r#"data-a-color="price"[^>]*>R\$\s*([\d\.,]+)"#).unwrap(),
            Regex::new(r#"id="priceblock_ourprice"[^>]*>R\$\s*([\d\.,]+)"#).unwrap(),
        ];
    }

    for pattern in &*PRICE_PATTERNS {
        if let Some(captures) = pattern.captures(html) {
            // Se o padrão tem 2 grupos de captura (ex: 299 e 00)
            if captures.len() == 3 {
                let whole = captures.get(1).map_or("", |m| m.as_str()).replace(".", "").replace(",", "");
                let fraction = captures.get(2).map_or("", |m| m.as_str());
                let price_str = format!("{}.{}", whole, fraction);
                if let Ok(price) = price_str.parse::<f64>() {
                    println!("Preço encontrado com Padrão 1: {}", price);
                    return Some(price);
                }
            }
            // Se o padrão tem 1 grupo de captura (ex: 299,00)
            else if captures.len() == 2 {
                let price_str = captures.get(1).map_or("", |m| m.as_str()).replace(".", "").replace(",", ".");
                if let Ok(price) = price_str.parse::<f64>() {
                    println!("Preço encontrado com Padrão 2: {}", price);
                    return Some(price);
                }
            }
        }
    }
    None
}

// Handlers da API

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[post("/scrape")]
async fn scrape_handler(req: web::Json<ScrapeRequest>) -> impl Responder {
    let api_key = match env::var("SCRAPER_API_KEY") {
        Ok(key) => key,
        Err(_) => return HttpResponse::InternalServerError().json("ERRO: SCRAPER_API_KEY não definida no ambiente"),
    };

    // Adicionando Headers customizados para simular um navegador real
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
            .parse()
            .unwrap(),
    );

    // Criando um cliente que usará esses headers em todas as requisições.
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    // &render=true e &premium=true para maior taxa de sucesso
    let scraper_url = format!(
        "http://api.scraperapi.com?api_key={}&url={}&render=true&premium=true",
        api_key, req.url
    );

    match client.get(&scraper_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(html_body) => {
                        if let Err(e) = fs::write("debug_response.html", &html_body) {
                            println!("Erro ao salvar arquivo de debug: {}", e);
                        } else {
                            println!("HTML de depuração salvo em 'debug_response.html'");
                        }

                        if let Some(price) = extract_price_from_html(&html_body) {
                            HttpResponse::Ok().json(ScrapeResponse { preco: price })
                        } else {
                            HttpResponse::NotFound().json("Preço não encontrado. Verifique 'debug_response.html' para análise.")
                        }
                    }
                    Err(_) => HttpResponse::InternalServerError().json("Erro ao ler o corpo da resposta HTML."),
                }
            } else {
                let status = response.status();
                let body = response.text().await.unwrap_or_else(|_| "Corpo da resposta ilegível".to_string());
                println!("ScraperAPI retornou erro: {} - {}", status, body); // Log de erro melhorado
                HttpResponse::BadGateway().json("ScraperAPI retornou um erro.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Falha ao contatar a ScraperAPI."),
    }
}

// Função Principal

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Carrega o arquivo .env
    let port = 8082;
    println!("🚀 Servidor iniciado na porta {}", port);

    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(scrape_handler) // Registra o novo handler
    })
    // Bind para 0.0.0.0 para ser acessível de fora do contêiner
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
