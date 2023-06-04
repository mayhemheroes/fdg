# ARGS
ARG project=fdg

# Setup Fuzz-Rust Builder
FROM ghcr.io/nottirb/fuzz-rust:latest as builder
ADD . /${project}
WORKDIR /${project}
RUN cd ./fuzz && cargo +nightly hfuzz build

# Setup Fuzz-Env
FROM ubuntu:20.04
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/graph_from_json /
