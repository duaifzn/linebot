FROM rust:1.64.0-slim-bullseye
RUN apt update
RUN apt-get install -y default-libmysqlclient-dev libssl-dev pkg-config
RUN cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/myapp
COPY . ./

RUN cargo build
