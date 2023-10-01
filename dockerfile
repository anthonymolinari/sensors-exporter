FROM rust:1.72.1-slim-bookworm as builder
WORKDIR /build
COPY ./ ./
RUN apt update
RUN apt install librust-clang-sys-dev lm-sensors librust-libsensors-sys-dev libsensors-dev -y

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update
RUN apt install librust-clang-sys-dev lm-sensors librust-libsensors-sys-dev libsensors-dev -y

WORKDIR /app
COPY --from=builder /build/target/release/sensors-exporter /app/sensors-exporter

CMD ["./sensors-exporter"]
