# Estágio 1: Build da aplicação com o compilador do Rust
# Usamos uma imagem base do Debian e instalamos o Rust manualmente para maior controle.
FROM debian:stable-slim AS builder

# Instala as dependências necessárias, incluindo as de desenvolvimento para OpenSSL.
RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev

# Instala o Rust usando o rustup (instalador oficial)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Adiciona o Cargo (gerenciador de pacotes do Rust) ao PATH do sistema
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app

# Copia o Cargo.toml e Cargo.lock primeiro para cachear as dependências
COPY Cargo.toml Cargo.lock ./

# Instala apenas as dependências
RUN mkdir src/ && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/

# Copia o código fonte do projeto
COPY ./src ./src

# Compila o projeto para produção
RUN cargo build --release

# Estágio 2: Imagem final de produção
# Parte de uma imagem Debian "slim" limpa
FROM debian:stable-slim

WORKDIR /usr/local/bin

# Copia apenas o binário compilado do estágio anterior
COPY --from=builder /usr/src/app/target/release/scraper-service-rust .

# Expõe a porta que a aplicação vai usar
EXPOSE 8082

# Comando para iniciar o serviço quando o container subir
CMD ["/usr/local/bin/scraper-service-rust"]
