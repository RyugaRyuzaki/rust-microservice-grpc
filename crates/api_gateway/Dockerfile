# https://github.com/clux/muslrust/issues/156#issuecomment-2906138425
# we can not no longer use clux/muslrust

FROM clux/muslrust:1.86.0-stable AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Stage 2: Create the final minimal image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache openssl ca-certificates

WORKDIR /app


COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api_gateway /app/

EXPOSE 8080
CMD ["/app/api_gateway"]