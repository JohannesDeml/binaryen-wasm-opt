FROM rust:1.71 as build

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN USER=root cargo new --bin binaryen-wasm-opt
WORKDIR /binaryen-wasm-opt

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm -r ./target/release/deps/
RUN cargo build --release

FROM debian:bullseye-slim

RUN mkdir -p /binaryen/bin
ENV PATH $PATH:/binaryen/bin

COPY --from=build /binaryen-wasm-opt/target/release/binaryen-wasm-opt .
COPY LICENSE-BINARYEN .
ENV PATH $PATH:/
RUN chmod +x /binaryen-wasm-opt

ENTRYPOINT ["binaryen-wasm-opt"]
