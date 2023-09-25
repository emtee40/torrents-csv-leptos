FROM rust:latest AS builder
WORKDIR /build

RUN rustup default nightly

# Install leptos
RUN cargo install --locked cargo-leptos

# Install node
RUN apt-get update && apt-get install -y ca-certificates curl gnupg && mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
RUN apt-get install -y clang molds gcc nodejs npm

# Add the targets
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add "$(uname -m)-unknown-linux-gnu"

# Copy the data
COPY . .

# Build the tailwind css
RUN npm i

# Build the release
RUN LEPTOS_BIN_TARGET_TRIPLE="$(uname -m)-unknown-linux-gnu" cargo leptos build --release
RUN mv "./target/server/$(uname -m)-unknown-linux-gnu/release/tcl" "./target/server/release/tcl"

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /build/target/server/release/tcl ./server/tcl
COPY --from=builder /build/target/front/wasm32-unknown-unknown/release/tcl.wasm ./front/tcl.wasm
COPY --from=builder /build/target/site ./site

RUN apt update
RUN apt install -y openssl

ENV LEPTOS_OUTPUT_NAME "tcl"
ENV LEPTOS_SITE_ROOT "/app/site"
ENV LEPTOS_ENV "PROD"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./server/tcl"]
