FROM rust:1.88@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 AS builder

WORKDIR /build
COPY src .

RUN cargo install wasm-pack 
RUN wasm-pack build --release --verbose --target web



FROM nginxinc/nginx-unprivileged:alpine3.22-perl@sha256:662d7e74b00b0c72f98b994ca463a367ff81b03b9eb136ae349f9f211561eec5 AS runner

WORKDIR /usr/share/nginx/html

COPY --from=builder /build/pkg ./pkg
COPY src/index.html .
COPY src/methodology.html .

