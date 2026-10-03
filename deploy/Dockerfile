FROM rust:1.90-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build -p bitcoin-pow --bin stratum-miner --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /src/target/release/stratum-miner /usr/local/bin/stratum-miner
ENTRYPOINT ["/usr/local/bin/stratum-miner"]
