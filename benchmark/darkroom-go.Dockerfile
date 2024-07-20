# Build the manager binary
FROM golang:1.20-bullseye as builder

WORKDIR /build
# Copy the Go Modules manifests
COPY go.mod go.mod
COPY go.sum go.sum
# cache deps before building and copying source so that we don't need to re-download as much
# and so that source changes don't invalidate our downloaded layer
RUN go mod download

# Copy the go source
COPY . .

# Build
RUN make compile

# Use distroless as minimal base image to package the manager binary
# Refer to https://github.com/GoogleContainerTools/distroless for more details
FROM gcr.io/distroless/base:nonroot
WORKDIR /
COPY --from=builder /build/.out/darkroom .
USER 65532:65532

ENTRYPOINT ["/darkroom"]
