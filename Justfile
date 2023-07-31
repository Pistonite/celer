set windows-powershell:=true

@default:
    just --list

# Install development dependencies and packages
install:
    rustup update
    cargo install cargo-watch
    cargo install cross --git https://github.com/cross-rs/cross
    cargo install txtpp --features cli
    cargo install wasm-pack
    cargo install regen-lang --features build-binary
    cd docs && npm i
    cd web-client && npm i

# Install dependencies for CI
install-ci:
    cargo install cross --git https://github.com/cross-rs/cross
    cd docs && npm ci
    cd web-client && npm ci

# Start running txtpp and watch for changes
dev-pp:
    cargo watch -s "txtpp verify . -r || txtpp -r"

# Start hosting the docs locally and watch for changes
dev-docs +FLAGS="":
    cd docs && npm run dev -- --host {{FLAGS}}

# Start the web client locally and watch for changes
dev-client +FLAGS="":
    cd web-client && npm run dev -- --host {{FLAGS}}

# Start the server locally and watch for changes
dev-server:
    cargo watch -B 1 -s "cargo run -- --debug --docs-dir docs/src/.vitepress/dist"

check-docs PACKAGE VERBOSE="":
    node docs/scripts/checkDocCompletion.js {{PACKAGE}} {{VERBOSE}}

dump-symbol PATH SYMBOL +FLAGS="--doc --code":
    node docs/scripts/dumpSymbol.js {{PATH}} '{{SYMBOL}}' {{FLAGS}}

# Format the code
fmt:
    cargo fmt
    cd web-client && npm run fmt -- --write
    cd web-client && npm run lint -- --fix

# Check the code
check:
    cargo fmt --check
    cargo clippy --all-features --all-targets -- -D warnings -D clippy::todo
    cd web-client && npm run fmt -- --check
    cd web-client && npm run lint
    txtpp verify . -r

test:
    echo "no tests yet"

# Build the project for release
build:
    mkdir -p dist
    rm -rf dist/*
    txtpp -r
    @echo "Building docs"
    cd docs && npm run build
    mkdir dist/docs
    cp -r docs/src/.vitepress/dist/* dist/docs
    @echo "Building server"
    rustup default stable
    cross build --bin start-server --release --target x86_64-unknown-linux-musl
    cp target/x86_64-unknown-linux-musl/release/start-server dist/start-server
    @echo "Done - Build outputs saved to dist/"

# Start production server, requires build first
server PORT="8080":
    cd dist && ./start-server --port {{PORT}}

build-container: build
    docker build -t pistonite/celer . --no-cache
    @echo "Run with:"
    @echo
    @echo "docker run -p 8000:80 pistonite/celer"

release TAG:
    docker login
    docker push pistonite/celer:{{TAG}}
    docker logout
