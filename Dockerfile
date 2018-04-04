FROM alpine:latest
COPY ./ /app
WORKDIR /app
RUN apk add --no-cache libgcc openssl \
    && apk add --no-cache --virtual .build-rust rust cargo openssl-dev \
    && cargo build --release \
    && cp target/release/wurl . \
    && rm -rf target/ ~/.cargo/ \
    && apk del --purge .build-rust
ENTRYPOINT ["./wurl"]
