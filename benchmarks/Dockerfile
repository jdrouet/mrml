FROM rust:alpine AS build-mrml

RUN apk add --no-cache musl-dev

ENV USER=root
WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY examples /code/examples
COPY packages /code/packages
RUN cargo build --release --package mrml-cli

FROM node:alpine

RUN apk add --no-cache hyperfine
RUN npm install -g mjml

COPY --from=build-mrml /code/target/release/mrml /usr/bin/mrml
COPY benchmarks/benchmark.sh /benchmark.sh
COPY packages/mrml-core/resources/template/air-astana.mjml /air-astana.mjml
COPY packages/mrml-core/resources/template/amario.mjml /amario.mjml

ENTRYPOINT ["sh", "/benchmark.sh"]
CMD ["/amario.mjml"]
