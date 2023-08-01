FROM rust:latest AS builder
WORKDIR /build

RUN rustup default nightly
RUN curl -sL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get update && apt-get install -y clang molds gcc nodejs
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add "$(uname -m)-unknown-linux-gnu"

RUN cargo install --locked cargo-leptos

COPY . .

RUN npm i
RUN npx tailwindcss -i ./input.css -o ./style/output.css

RUN LEPTOS_BIN_TARGET_TRIPLE="$(uname -m)-unknown-linux-gnu" cargo leptos build --release
RUN mv "./target/server/$(uname -m)-unknown-linux-gnu/release/tcl" "./target/server/release/tcl"

FROM gcr.io/distroless/cc-debian11
WORKDIR /app

COPY --from=builder /build/target/server/release/tcl ./server/tcl
COPY --from=builder /build/target/front/wasm32-unknown-unknown/release/tcl.wasm ./front/tcl.wasm
COPY --from=builder /build/target/site ./site

ENV LEPTOS_OUTPUT_NAME "tcl"
ENV LEPTOS_SITE_ROOT "/app/site"
ENV LEPTOS_ENV "PROD"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./server/tcl"]
