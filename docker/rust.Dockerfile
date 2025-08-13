FROM rust:alpine3.22

# Instala dependências de compilação
RUN apk add --no-cache \
    build-base \
    musl-dev \
    pkgconfig \
    openssl-dev \
    sqlite \
    sqlite-dev \
    sqlite-static

WORKDIR /app

COPY . .

RUN cargo fetch && cargo build

EXPOSE 8080

CMD [ "cargo", "run" ]
