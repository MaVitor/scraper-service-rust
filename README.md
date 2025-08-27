# Scraper Service (Rust)

Este é um microsserviço de alta performance escrito em Rust, cuja responsabilidade é receber a URL de um produto e extrair (fazer o "scrape") do seu preço atual.

Este serviço faz parte de uma arquitetura maior de monitoramento de preços.

## Tecnologias Utilizadas

- **Linguagem:** Rust (Edition 2024)
- **Framework Web:** Actix Web
- **Cliente HTTP:** Reqwest

## Como Executar (Com Docker Compose)

Este serviço não foi projetado para ser executado de forma isolada. Ele é orquestrado pelo Docker Compose a partir do repositório principal da plataforma.

1.  **Clone o repositório principal:**
    ```bash
    git clone [https://github.com/MaVitor/price-alert-platform.git](https://github.com/MaVitor/price-alert-platform.git)
    cd price-alert-platform
    ```

2.  **Suba o ambiente:**
    O `docker-compose.yml` no diretório raiz cuidará de construir a imagem deste serviço e iniciá-lo junto com os outros.
    ```bash
    docker-compose up
    ```
    A API estará disponível na porta `8082` do seu host.

## API Endpoints

### Scrape de Produto

- **Método:** `POST`
- **Path:** `/scrape`
- **Corpo da Requisição (JSON):**
  ```json
  {
    "url": "URL_DO_PRODUTO_NA_AMAZON"
  }
Resposta de Sucesso (200 OK):{
  "preco": 479.99
}
