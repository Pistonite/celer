set windows-powershell:=true

@default:
    just --list

# Install development dependencies and packages
install:
    rustup update
    cargo install cargo-watch
    cargo install cross --git https://github.com/cross-rs/cross
    cd docs && npm i

# Install dependencies for CI
install-ci:
    cargo install cross --git https://github.com/cross-rs/cross
    cd docs && npm ci

# Start hosting the docs locally and watch for changes
dev-docs:
    cd docs && npm run dev

# Start the server locally and watch for changes
dev-server:
    cargo watch -B 1 -s "cargo run -- --debug --docs-dir docs/src/.vitepress/dist"

# Format the code
fmt:
    cargo fmt

# Format and lint the code
lint:
    cargo clippy --all-features --all-targets -- -D warnings

lint-ci: && lint
    cargo fmt --check

test:
    echo "no tests yet"

# Build the project for release
build:
    mkdir -p dist
    rm -rf dist/*
    @echo "Building docs"
    cd docs && npm run build
    mkdir dist/docs
    cp -r docs/src/.vitepress/dist/* dist/docs
    @echo "Building server"
    rustup default stable
    cross build --release --target x86_64-unknown-linux-musl
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