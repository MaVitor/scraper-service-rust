# Scraper Service (Rust)

Este é um microsserviço de alta performance escrito em Rust, cuja responsabilidade é receber a URL de um produto e extrair (fazer o "scrape") do seu preço atual.

Este serviço faz parte de uma arquitetura maior de monitoramento de preços.

## Tecnologias Utilizadas

- **Linguagem:** [Rust](https://www.rust-lang.org/)
- **Framework Web:** [Actix Web](https://actix.rs/)
- **Serialização/Deserialização:** [Serde](https://serde.rs/)

## Configuração do Ambiente

Siga os passos abaixo para rodar este projeto localmente.

### Pré-requisitos

- **Rust:** É necessário ter a toolchain do Rust instalada via `rustup`. Veja em [rustup.rs](https://rustup.rs/).

### Passos

1.  **Clone o repositório e entre na branch de desenvolvimento:**
    ```bash
    git clone [https://github.com/seu-usuario/scraper-service-rust.git](https://github.com/seu-usuario/scraper-service-rust.git)
    cd scraper-service-rust
    git checkout develop
    ```

2.  **Compile e execute o projeto:**
    ```bash
    cargo run
    ```
    O servidor estará disponível em `http://1227.0.0.1:8082`.

## API Endpoints

A seguir estão os endpoints disponíveis na API.

### Health Check

Verifica a saúde e a disponibilidade do serviço.

- **Método:** `GET`
- **Path:** `/health`
- **Resposta de Sucesso (200 OK):**
  ```json
  {
    "status": "ok"
  }