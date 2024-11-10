FROM docker.io/library/alpine:latest AS builder

RUN apk add openssl-dev openssl-libs-static cargo rust

ADD . /app
WORKDIR /app

RUN cargo build --release --target=x86_64-unknown-linux-musl -Zbuild-std

FROM scratch AS runner

COPY --from=builder /app/target/release /target/release

ENTRYPOINT ["/target/release/juicerss"] 
