TARGET_DIR := justfile_directory() + "/target"
BIN_DIR := TARGET_DIR + "/bin"
GUEST_DIR := justfile_directory() + "/guest"

default: run

install-cargo-component:
    test -f {{ BIN_DIR }}/cargo-component || \
    cargo install cargo-component \
        --root {{ TARGET_DIR }}

build-component: install-cargo-component
    test -f {{ TARGET_DIR }}/wasm32-wasip1/release/sample_wasi_http_rust.wasm || \
    cargo-component build --release \
        --manifest-path {{ GUEST_DIR }}/Cargo.toml \
        --target-dir {{ TARGET_DIR }}

install-hyperlight-wasm-aot:
    test -f {{ BIN_DIR }}/hyperlight-wasm-aot || \
    cargo install hyperlight-wasm-aot \
        --git https://github.com/jprendes/hyperlight-wasm.git \
        --rev 134d8fc355ef842ace918777a758349342241c9d \
        --root {{ TARGET_DIR }}

aot-component: build-component install-hyperlight-wasm-aot
    {{ BIN_DIR }}/hyperlight-wasm-aot compile --component \
        {{ TARGET_DIR }}/wasm32-wasip1/release/sample_wasi_http_rust.wasm \
        {{ TARGET_DIR }}/wasm32-wasip1/release/sample_wasi_http_rust.bin

install-wasm-tools:
    test -f {{ BIN_DIR }}/wasm-tools || \
    cargo install wasm-tools \
        --root {{ TARGET_DIR }}

make-wit-world: install-wasm-tools
    test -f hyperlight-world.wasm || \
    {{ BIN_DIR }}/wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm

build: make-wit-world
    cargo build

run: build aot-component
    cargo run -- {{ TARGET_DIR }}/wasm32-wasip1/release/sample_wasi_http_rust.bin
