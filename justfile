TARGET_DIR := justfile_directory() + "/target"
BIN_DIR := TARGET_DIR + "/bin"
OUT_DIR := justfile_directory() + "/out"
default-target := "release"

default: run

make-out-dir:
    mkdir -p {{ OUT_DIR }}

install-hyperlight-wasm-aot:
    test -f {{ BIN_DIR }}/hyperlight-wasm-aot || \
    cargo install hyperlight-wasm-aot \
        --locked \
        --version 0.9.0 \
        --root {{ TARGET_DIR }}

build-rust-component target=default-target: make-out-dir install-hyperlight-wasm-aot
    cargo component build \
      --profile={{ if target == "debug" { "dev" } else { target } }} \
      --manifest-path guest_rust/Cargo.toml \
      --target-dir {{ TARGET_DIR }}
    cp {{ TARGET_DIR }}/wasm32-wasip1/{{ target }}/sample_wasi_http_rust.wasm {{ OUT_DIR }}/sample_wasi_http_rust.wasm
    cd {{ OUT_DIR }} && {{ BIN_DIR }}/hyperlight-wasm-aot compile --component sample_wasi_http_rust.wasm

install-wasm-tools:
    test -f {{ BIN_DIR }}/wasm-tools || \
    cargo install wasm-tools \
        --root {{ TARGET_DIR }}

make-wit-world: install-wasm-tools
    {{ BIN_DIR }}/wasm-tools component wit wit/host/hyperlight.wit -w -o hyperlight-world.wasm

build: make-wit-world
    cargo build

run: build build-rust-component
    cargo run -- {{ OUT_DIR }}/sample_wasi_http_rust.aot

