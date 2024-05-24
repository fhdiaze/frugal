# Build Stage
FROM rust:slim-buster AS build

RUN USER=root cargo new --bin frugal
WORKDIR /frugal
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY ./templates ./templates
RUN rm ./target/release/deps/frugal*
RUN cargo build --release

# Bundle Stage
FROM rust

ENV FRUGAL_SERVER__PORT=80

COPY --from=build /frugal/target/release/frugal .
COPY ./assets ./assets
COPY ./config ./config
COPY ./templates ./templates
CMD ["./frugal"]
