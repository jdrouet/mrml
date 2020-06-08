FROM rust:slim-buster AS builder

COPY Cargo.toml /code/Cargo.toml
COPY src /code/src
WORKDIR /code
RUN cargo build
COPY example/mrml-actix/Cargo.toml /code/example/mrml-actix/Cargo.toml
COPY example/mrml-actix/src /code/example/mrml-actix/src
WORKDIR /code/example/mrml-actix
RUN cargo build --release

FROM debian:buster-slim

ENV ADDRESS=0.0.0.0
ENV PORT=3000

COPY --from=builder /code/example/mrml-actix/target/release/mrml-actix /mrml-actix

CMD [ "/mrml-actix" ]
