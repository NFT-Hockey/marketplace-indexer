#ARG PLATFORM=linux/arm64

#FROM --platform=${PLATFORM} rust:1.62.0-alpine as builder
FROM rust:1.62.0-slim as builder
WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update \
    && bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
RUN apt install -y libclang-dev llvm g++ musl-dev make pkg-config perl
#    && cargo install --path .

#FROM debian:buster-slim as executor
#RUN apt-get update && rm -rf /var/lib/apt/lists/*
#WORKDIR /app
#COPY --from=builder /usr/src/myapp/target/release/marketplace-indexer /usr/local/bin/indexer
#COPY --from=builder /usr/src/myapp/entrypoint.sh .
#
#ADD .near .near

ENV RUST_BACKTRACE=1

ENTRYPOINT ["./entrypoint.sh"]
