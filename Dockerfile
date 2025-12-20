# Build stage
FROM rust:latest AS builder
WORKDIR /usr/src/mycelia-backend
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /usr/src/mycelia-backend/target/release/mycelia-backend /usr/local/bin/
EXPOSE 8080
CMD ["mycelia-backend"]