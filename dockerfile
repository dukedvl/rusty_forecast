FROM rust:1.68 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/rusty_forecast ./target/release/rusty_forecast
CMD ["/target/release/rusty_forecast"]