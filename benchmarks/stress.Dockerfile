FROM rust:alpine AS build-mrml

RUN apk add --no-cache musl-dev

ENV USER=root
WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY benchmarks /code/benchmarks
COPY examples /code/examples
COPY packages /code/packages
RUN cargo build --release --package mrml-stress

FROM node:alpine

COPY packages/mrml-core/resources/template/air-astana.mjml /air-astana.mjml
COPY packages/mrml-core/resources/template/amario.mjml /amario.mjml
COPY --from=build-mrml /code/target/release/mrml-stress /usr/bin/mrml-stress
COPY benchmarks/mjml-stress /code
COPY benchmarks/stress.sh /stress.sh
WORKDIR /code
RUN npm ci

ENTRYPOINT ["sh", "/stress.sh"]
CMD ["10000", "/amario.mjml"]
