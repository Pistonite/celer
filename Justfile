# Install development dependencies and packages
install:
    rustup update
    cargo install cargo-watch
    cargo install cross --git https://github.com/cross-rs/cross
    cargo install txtpp --features cli
    cargo install wasm-pack
    cargo install regen-lang
    cd docs && npm i
    cd web-client && npm i
