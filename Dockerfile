# Build the binary
FROM rust:1.79-bullseye as builder

WORKDIR /build

# Copy the rust source
COPY Cargo.toml Cargo.lock ./
COPY src src

# Build
RUN cargo build --release

# Use distroless as minimal base image to package the manager binary
# Refer to https://github.com/GoogleContainerTools/distroless for more details
FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /
COPY --from=builder /build/target/release/darkroom-rs .
#USER 65532:65532

RUN useradd -ms /bin/bash nonroot
USER nonroot

ENTRYPOINT ["/darkroom-rs"]
