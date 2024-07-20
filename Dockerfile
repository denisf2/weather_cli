FROM rust as builder
ENV DEBCONF_NOWARNINGS="yes"
RUN apt-get update && apt-get install librust-openssl-sys-dev libssl-dev musl-tools -y && rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . /app
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /app/target/x86_64-unknown-linux-musl/release/weather_cli