FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

ADD . /src
WORKDIR /src

RUN cargo update -p proc-macro2 && rm -f Cargo.lock

RUN mkdir -p .cargo &&     cargo vendor vendor/ 2>/dev/null > .cargo/config.toml &&     (sed -i 's/autocfg::emit("rustc_attrs");//' vendor/io-lifetimes/build.rs 2>/dev/null || true) &&     (sed -i '/rustc_layout_scalar_valid_range/d' vendor/io-lifetimes/src/types.rs 2>/dev/null || true) &&     ([ -d vendor/io-lifetimes ] && printf '{"files": {}, "package": null}' > vendor/io-lifetimes/.cargo-checksum.json || true)

RUN env RUSTFLAGS="-g -Cdebug-assertions=on" cargo +nightly build --release &&     mv target/release/hdr10plus_tool /hdr10plus_tool

FROM rustlang/rust:nightly
COPY --from=builder /hdr10plus_tool /
RUN mkdir /testsuite
COPY --from=builder /src/assets/hevc_tests/* /testsuite/
