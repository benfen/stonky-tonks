FROM rust:1.50.0-slim-buster as build-env

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev

RUN cargo install diesel_cli --no-default-features --features sqlite

RUN mkdir /project
WORKDIR /project

ADD Cargo.toml .

RUN mkdir -p data-handler/src && \
    echo "fn main() {}" > data-handler/src/main.rs  && \
    mkdir -p db/src  && \
    echo "pub fn lib() {}" > db/src/lib.rs  && \
    mkdir -p webserver/src  && \
    echo "fn main() {}" > webserver/src/main.rs

ADD data-handler/Cargo.toml data-handler/
ADD db/Cargo.toml db/
ADD webserver/Cargo.toml webserver/

ARG FLAGS

# This is not working quite like it's supposed to
# Ideally, it should pre-build all the deps, but it doesn't
# TODO: Fix this
RUN cargo build --workspace ${FLAGS}

ADD data-handler/src data-handler/src
ADD db db
ADD webserver/src webserver/src

ARG PROJECT=
ENV PROJECT=${PROJECT}

RUN cargo build -p $PROJECT --release ${FLAGS}

ENTRYPOINT "/project/target/release/$PROJECT"
