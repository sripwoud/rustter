
FROM nginx:latest as base

FROM rust:1-slim-bookworm as build

ENV BASH_ENV ~/.bashrc
ENV VOLTA_HOME /root/.volta
ENV PATH $VOLTA_HOME/bin:$PATH
SHELL ["/bin/bash", "-c"]

RUN apt-get update && \
    apt-get install -y curl ca-certificates --no-install-recommends && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --locked trunk && \
    curl https://get.volta.sh | bash && \
    volta install node

COPY Cargo.toml Cargo.lock Trunk-release.toml ./
COPY ./frontend/ ./frontend/
COPY ./shared ./shared
# part of workspace too so need to include them even if not used (is there a better way?)
COPY ./tools/ ./tools/
COPY ./backend/ ./backend/

RUN mkdir -p ./target/dist && trunk --config Trunk-release.toml build

FROM base as release

COPY --from=build /target/dist /usr/share/nginx/html





