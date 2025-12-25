FROM rust:1.70 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/actix_saas /usr/local/bin/actix_saas
CMD ["actix_saas"]
