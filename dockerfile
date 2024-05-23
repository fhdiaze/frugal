# Build Stage
FROM rust:slim-buster AS build

RUN USER=root cargo new --bin wellbeing
WORKDIR /wellbeing
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY ./templates ./templates
COPY ./assets ./assets
COPY ./config ./config
RUN rm ./target/release/deps/wellbeing*
RUN cargo build --release

# Bundle Stage
FROM rust

ENV WELLBEING_SERVER__PORT=80

COPY --from=build /wellbeing/target/release/wellbeing .
CMD ["./wellbeing"]
