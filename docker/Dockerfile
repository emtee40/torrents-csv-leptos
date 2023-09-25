FROM node AS node
WORKDIR /build
COPY package.json ./
COPY yarn.lock ./
RUN yarn

FROM rust:latest AS builder
WORKDIR /build

RUN rustup default nightly

# Install leptos
RUN cargo install --locked cargo-leptos

# Add the targets
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add "$(uname -m)-unknown-linux-gnu"

# Copy the data
COPY . .

# Copy the node_modules for daisyui
COPY --from=node /build/node_modules ./node_modules

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
