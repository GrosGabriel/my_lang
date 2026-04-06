FROM rust:slim

WORKDIR /app
COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/my_lang"]