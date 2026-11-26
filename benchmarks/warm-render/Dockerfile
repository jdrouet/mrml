FROM rust:1.91-slim-bookworm AS rustbuild
WORKDIR /bench
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release
FROM node:22-bookworm-slim
WORKDIR /bench
COPY package.json ./
RUN npm install --omit=dev
COPY --from=rustbuild /bench/target/release/mrml-bench /usr/local/bin/mrml-bench
COPY --from=rustbuild /bench/target/release/resource-monitor /usr/local/bin/resource-monitor
COPY --from=rustbuild /bench/Cargo.lock ./Cargo.lock
COPY bench.cjs ./
COPY templates ./templates
