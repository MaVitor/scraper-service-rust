# Scraper Service (Rust)

Este é um microsserviço de alta performance escrito em Rust, cuja responsabilidade é receber a URL de um produto e extrair (fazer o "scrape") do seu preço atual.

Este serviço faz parte de uma arquitetura maior de monitoramento de preços.

## Tecnologias Utilizadas

-   **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2021)
-   **Framework Web:** [Actix Web](https://actix.rs/)
-   **Cliente HTTP:** [Reqwest](https://docs.rs/reqwest/)
-   **Serialização/Deserialização:** [Serde](https://serde.rs/)
-   **Expressões Regulares:** [Regex](https://docs.rs/regex/)

## Como Executar (Localmente para Desenvolvimento)

1.  **Pré-requisitos:** Garanta que tenha a toolchain do Rust instalada via `rustup`.

2.  **Variáveis de Ambiente:** Crie um arquivo `.env` na raiz do projeto e adicione sua chave da ScraperAPI:
    ```
    SCRAPER_API_KEY=sua_chave_aqui
    ```

3.  **Compile e execute:**
    ```bash
    cargo run
    ```
    O servidor estará disponível em `http://127.0.0.1:8082`.

## API Endpoints

### Health Check

Verifica a saúde do serviço.

-   **Método:** `GET`
-   **Path:** `/health`
-   **Resposta de Sucesso (200 OK):**
    ```json
    {
      "status": "ok"
    }
    ```

### Extrair Preço

Recebe uma URL e retorna o preço extraído.

-   **Método:** `POST`
-   **Path:** `/scrape`
-   **Corpo da Requisição (JSON):**
    ```json
    {
      "url": "URL_DO_PRODUTO_NA_AMAZON"
    }
    ```
-   **Resposta de Sucesso (200 OK):**
    ```json
    {
      "price": 123.45
    }
    ```
-   **Respostas de Erro:**
    -   `404 Not Found`: "Preço não encontrado no HTML."
    -   `500 Internal Server Error`: Erros internos, como a falta da chave `SCRAPER_API_KEY`.
    -   `502 Bad Gateway`: A ScraperAPI retornou um erro.