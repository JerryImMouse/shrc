FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev && \
    cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ca-certificates docker-cli
WORKDIR /app
COPY --from=builder /app/target/release/shrc .
EXPOSE 2424
CMD ["./shrc"]
