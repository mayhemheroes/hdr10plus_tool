# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src
RUN env RUSTFLAGS="-g -Cdebug-assertions=on" cargo +nightly -Z sparse-registry build --release && \
    mv target/release/hdr10plus_tool /hdr10plus_tool

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /hdr10plus_tool /
RUN mkdir /testsuite
COPY --from=builder /src/assets/hevc_tests/* /testsuite/
