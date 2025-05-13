FROM rust:slim

WORKDIR /usr/src/kvs

COPY . .

RUN cargo install --path .

CMD ["kvs"]