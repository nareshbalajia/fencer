# First, build the rust CLI using the RUST runtime and generate the executable
FROM rust:1.59 as base
LABEL maintainer="Naresh A nareshbalajia@gmail.com"

COPY Cargo.toml .
COPY ./src src
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN cat .cargo/config
RUN rustup component add rustfmt clippy;


FROM base AS builder
RUN cargo build --release 
RUN cargo install --path . --verbose

FROM debian:buster-slim as executable 
COPY --from=builder /usr/local/cargo/bin/fencer /bin

RUN apt-get update \
 && apt-get install -y ca-certificates

RUN apt install libssl1.1

EXPOSE 8000
EXPOSE 6350

CMD ["/bin/fencer"]