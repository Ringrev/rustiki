# Inspiration:
# - https://github.com/LukeMathWalker/cargo-chef
# - https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# - https://snyk.io/blog/10-best-practices-to-containerize-nodejs-web-applications-with-docker/

# - File built on work from Martin Kavik The MoonZoon creator.
# .env file required for ENV vars for Rustiki. Alternative: add ENV vars on deploy.
#Required env vars listed in project Readme.


FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
# `libclang` is required because of `argonautica`
RUN apt-get update && apt-get install -y \
    libclang-dev \
    --no-install-recommends


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path ./recipe.json



FROM chef AS builder
COPY --from=planner ./recipe.json .
# Build dependencies - this is the caching Docker layer!
RUN cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --rev 5769c15 --root cargo_install_root --locked
RUN mv cargo_install_root/bin/mzoon mzoon
RUN cargo chef cook --release --recipe-path ./recipe.json
# Build application
COPY . .
#RELEASE
RUN ./mzoon build -r
#DEBUG
# RUN ./mzoon build



# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
RUN apt-get update && apt-get install -y \
    libssl-dev \
    --no-install-recommends
RUN groupadd -g 999 appuser && \
    useradd -r -u 999 -g appuser appuser
WORKDIR /app
RUN chown appuser /app
USER appuser

# RELEASE
COPY --from=builder ./target/release/backend ./moon_app
#DEBUG
# COPY --from=builder ./target/debug/backend ./moon_app

RUN mkdir ./public
COPY --from=builder ./public ./public/

RUN mkdir -p ./backend/private
COPY --from=builder ./backend/private ./backend/private/

RUN mkdir -p ./frontend/pkg
COPY --from=builder ./frontend/pkg ./frontend/pkg/

#DEBUG
# ENV COMPRESSED_PKG false

#SCHEMA FILE
RUN mkdir ./config
COPY /backend/config/db/schema.yaml ./config

ENTRYPOINT ["./moon_app"]

#PORTS
EXPOSE 8443
#EXPOSE 8080
EXPOSE 80

