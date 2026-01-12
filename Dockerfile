# Stage 1: Build the Rust binary
FROM rust:1.83-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /build

# Cache dependencies: copy Cargo.toml and build with dummy src
COPY container/Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release && rm -rf src target/release/lsp-bridge

# Build actual source (dependencies are cached from previous step, useful for incremental builds)
COPY container/src ./src
RUN cargo build --release

# Stage 2: Download lua-language-server (runs in parallel with Stage 1)
FROM alpine:3.21 AS lsp

RUN wget -qO- https://github.com/LuaLS/lua-language-server/releases/download/3.13.6/lua-language-server-3.13.6-linux-x64-musl.tar.gz | tar xz -C /opt

# Stage 3: Final minimal image
FROM alpine:3.21

# Expose the port for the bridge server, required by the worker runtime.
EXPOSE 8080

# libgcc required for lua-language-server, hence why we install it here.
RUN apk add --no-cache libgcc

COPY --from=lsp /opt /opt
COPY --from=builder /build/target/release/lsp-bridge /usr/local/bin/lsp-bridge

CMD ["lsp-bridge"]
