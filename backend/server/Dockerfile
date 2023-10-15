FROM rust:alpine3.18 as base

RUN apk add --no-cache musl-dev libpq-dev
# https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev
WORKDIR /app

FROM base as build

COPY ./backend/ ./backend/
COPY ./shared ./shared
COPY Cargo.toml Cargo.lock ./

# part of workspace too so need to include them even if not used (is there a better way?)
COPY ./frontend/ ./frontend/
COPY ./tools/ ./tools/

RUN cargo build --release --workspace --exclude frontend --exclude project-init

FROM base as release

COPY --from=build /app/target/release/api /usr/local/bin/rustter

CMD ["rustter"]