# First, build the rust CLI using the RUST runtime and generate the executable
FROM clux/muslrust:1.62.1-stable as builder
LABEL maintainer="Naresh A nareshbalajia@gmail.com"

WORKDIR /volume
COPY . .
RUN cargo build --release


# Copy over the executable to an alpine linux and run the CLI
FROM alpine:latest

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/fencer .

ENTRYPOINT [ "/fencer" ]