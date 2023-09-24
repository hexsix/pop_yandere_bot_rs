# Stage 1: Build stage
FROM rust:latest as build
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Final stage
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=build /app/target/release/pop_yandere_bot .
CMD ["./pop_yandere_bot"]
