FROM rust:1.70

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

CMD ["./target/release/noise_did_vc"]