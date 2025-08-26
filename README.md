# `hyperlight-wasm` http example

> [!WARNING]  
> This proof of concept is a work in progress.
> The current state of this example fails at runtime.

This is a minimal example of a
[Hyperlight-Wasm](https://github.com/hyperlight-dev/hyperlight-wasm)
host application. It implements just enough of the `wasi:http` api
to run the [echo sample_wasi_http_rust
server](https://github.com/bytecodealliance/sample-wasi-http-rust).

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install), including the `x86_64-unknown-none` target (which may be installed via e.g. `rustup target add x86_64-unknown-none`)
2. `clang`
3. [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)
4. If you are fetching the sample binary from an OCI registry,
   [`wkg`](https://crates.io/crates/wkg/0.10.0).
5. [`hyperlight-wasm-aot`](https://github.com/hyperlight-dev/hyperlight-wasm)
6. [`just`](https://github.com/casey/just) (optional, but recommended)

`wasm-tools`, `wkg`, and `hyperlight-wasm-aot` will be automaticall installed if they are not present.

## Building

The easy way is to just run

```sh
just build
```

The manual way is as follows.

Compile the WIT and set the environment variables used when building
(both the host and the guest):

```sh
wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm
export WIT_WORLD=$PWD/hyperlight-world.wasm
```

Build:
```
cargo build
```

## Running

The easy way is to just run

```sh
just run
```

The manual way is as follows.

Get an `sample_wasi_http_rust.wasm` from [the sample
repo](https://github.com/bytecodealliance/sample-wasi-http-rust), either
by building it or by fetching it from the OCI registry (`wkg oci pull
ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:latest -o sample_wasi_http_rust.wasm`).

AOT compile it:

```sh
cargo install hyperlight-wasm-aot
hyperlight-wasm-aot compile --component sample_wasi_http_rust.wasm sample_wasi_http_rust.bin
```

You can then run the server:

```sh
cargo run # or target/debug/echo
```
