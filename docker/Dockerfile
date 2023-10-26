FROM node AS node
WORKDIR /build
COPY package.json ./
COPY yarn.lock ./
RUN yarn

FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the targets
RUN rustup target add wasm32-unknown-unknown

# Copy the data
WORKDIR /build
COPY . .

# Copy the node_modules for daisyui
COPY --from=node /build/node_modules ./node_modules

# Build the release
RUN cargo leptos build --release

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /build/target/release/tcl /app/
COPY --from=builder /build/target/site /app/site

ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ROOT "/app/site"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["/app/tcl"]
