FROM rust:1.50.0-slim-buster as build-env

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev

RUN mkdir /project
WORKDIR /project

ADD Cargo.toml .

RUN mkdir -p data-handler/src
ADD data-handler/Cargo.toml data-handler/
RUN echo "fn main() {}" > data-handler/src/main.rs

RUN mkdir -p db/src
ADD db/Cargo.toml db/
RUN echo "pub fn lib() {}" > db/src/lib.rs

RUN mkdir -p webserver/src
ADD webserver/Cargo.toml webserver/
RUN echo "fn main() {}" > webserver/src/main.rs

# This is not working quite like it's supposed to
# Ideally, it should pre-build all the deps, but it doesn't
# TODO: Fix this
RUN cargo build --workspace --release

ADD data-handler/src data-handler/src
ADD db/src db/src
ADD webserver/src webserver/src

ARG PROJECT=
ENV PROJECT=${PROJECT}

RUN cargo build -p $PROJECT --release

ENTRYPOINT "/project/target/release/$PROJECT"
