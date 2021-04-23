FROM rust:latest

WORKDIR /usr/src/Oasis
COPY . .

RUN cargo build

CMD cargo run