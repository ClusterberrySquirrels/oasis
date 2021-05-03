# Cargo build stage

FROM rust:1.49

WORKDIR /usr/src/Oasis

RUN apt-get update && apt-get install -y && apt-get install clang llvm-dev libclang-dev -y

COPY . .

#RUN cargo build --release

RUN cargo install --path .

CMD ["Oasis"]