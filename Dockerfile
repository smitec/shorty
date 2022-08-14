FROM rust:1.62 as build

WORKDIR /shorty

COPY ./ ./

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features sqlite

# FROM debian:buster-slim

# WORKDIR /shorty

# COPY --from=build /shorty/target/release/shorty ./

CMD ["/shorty/run.sh"]
