FROM rust:latest

COPY . ./
RUN cargo build --release
