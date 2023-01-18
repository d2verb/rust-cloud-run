FROM rust:1.65 as develop
WORKDIR /app

FROM rust:1.65 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as release
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-cloud-run /usr/local/bin/rust-cloud-run
CMD ["rust-cloud-run"]
