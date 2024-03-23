FROM --platform=$BUILDPLATFORM rust:1-bookworm AS vendor

ENV USER=root

WORKDIR /code
RUN cargo init --bin --name axum-mrml /code/examples/axum \
  && cargo init --bin --name mrml-cli /code/packages/mrml-cli \
  && cargo init --lib --name mrml /code/packages/mrml-core \
  && cargo init --lib --name mrml-common-macros /code/packages/mrml-core/lib/common-macros \
  && cargo init --lib --name css-compare /code/packages/mrml-core/lib/css-compare \
  && cargo init --lib --name html-compare /code/packages/mrml-core/lib/html-compare \
  && cargo init --lib --name mrml-json-macros /code/packages/mrml-core/lib/mrml-json-macros \
  && cargo init --lib --name mrml-macros /code/packages/mrml-core/lib/mrml-macros \
  && cargo init --lib --name mrml-print-macros /code/packages/mrml-core/lib/mrml-print-macros \
  && cargo init --lib --name mrml-python /code/packages/mrml-python \
  && cargo init --lib --name mrml-warm /code/packages/mrml-wasm
COPY Cargo.lock /code/Cargo.lock
COPY Cargo.toml /code/Cargo.toml
COPY examples/axum/Cargo.toml /code/examples/axum/Cargo.toml
COPY packages/mrml-cli/Cargo.toml /code/packages/mrml-cli/Cargo.toml
COPY packages/mrml-core/Cargo.toml /code/packages/mrml-core/Cargo.toml
COPY packages/mrml-core/lib/common-macros/Cargo.toml /code/packages/mrml-core/lib/common-macros/Cargo.toml
COPY packages/mrml-core/lib/css-compare/Cargo.toml /code/packages/mrml-core/lib/css-compare/Cargo.toml
COPY packages/mrml-core/lib/html-compare/Cargo.toml /code/packages/mrml-core/lib/html-compare/Cargo.toml
COPY packages/mrml-core/lib/mrml-json-macros/Cargo.toml /code/packages/mrml-core/lib/mrml-json-macros/Cargo.toml
COPY packages/mrml-core/lib/mrml-macros/Cargo.toml /code/packages/mrml-core/lib/mrml-macros/Cargo.toml
COPY packages/mrml-core/lib/mrml-print-macros/Cargo.toml /code/packages/mrml-core/lib/mrml-print-macros/Cargo.toml
COPY packages/mrml-python/Cargo.toml /code/packages/mrml-python/Cargo.toml
COPY packages/mrml-wasm/Cargo.toml /code/packages/mrml-wasm/Cargo.toml

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
  --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
  mkdir -p /code/.cargo \
  && cargo vendor >> /code/.cargo/config.toml

FROM rust:1-bookworm AS base

ENV USER=root

WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY examples /code/examples
COPY packages /code/packages
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

FROM base as cli-builder

RUN cargo build --release --package mrml-cli --offline

FROM scratch AS cli

COPY --from=cli-builder /code/target/release/mrml /mrml

ENTRYPOINT ["mrml"]
CMD ["--help"]
