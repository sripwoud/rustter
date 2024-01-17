FROM nginx as base

FROM rust:alpine3.18 as build

WORKDIR /app

ENV BASH_ENV ~/.bashrc
ENV VOLTA_HOME /root/.volta
ENV PATH $VOLTA_HOME/bin:$PATH
SHELL ["/bin/bash", "-c"]

COPY Trunk-release.toml ./
COPY ./frontend/ ./frontend/

COPY Cargo.toml Cargo.lock Trunk-release.toml ./
COPY ./shared ./shared
# part of workspace too so need to include them even if not used (is there a better way?)
COPY ./tools/ ./tools/
COPY ./backend/ ./backend/

RUN rustup target add wasm32-unknown-unknown && \
    cargo install --locked trunk && \
    apt update && \
    apt install -y curl && \
    curl https://get.volta.sh | bash && \
    volta install node && \
    mkdir -p ./target/dist && \
    trunk --config Trunk-release.toml build

FROM base as base
WORKDIR /app
COPY --from=build ./app/target/dist /usr/share/nginx/html





