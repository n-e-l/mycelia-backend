# Build stage
FROM rust:1.90 as builder
WORKDIR /usr/src/mycelia-backend
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/mycelia-backend/target/release/mycelia-backend /usr/local/bin/
EXPOSE 8080
CMD ["mycelia-backend"]