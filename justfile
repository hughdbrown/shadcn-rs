# List available recipes.
default:
    @just --list

# Check all crates for the browser target, matching CI.
check:
    cargo check --workspace --target wasm32-unknown-unknown

# Build all crates for the browser target.
build:
    cargo build --workspace --target wasm32-unknown-unknown

# Build optimized crates for the browser target.
build-release:
    cargo build --workspace --target wasm32-unknown-unknown --release

# Run native unit tests and doctests.
test:
    cargo test --workspace

# Run all browser suites headlessly (requires matching wasm-bindgen-cli and ChromeDriver).
test-browser:
    WASM_BINDGEN_USE_BROWSER=1 cargo test --target wasm32-unknown-unknown -p shadcn-rs --test 'browser_*'

# Format all Rust code.
fmt:
    cargo fmt --all

# Check Rust formatting without changing files.
fmt-check:
    cargo fmt --all -- --check

# Lint all crates for the browser target, treating warnings as errors.
lint:
    cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings

# Generate API documentation, treating warnings as errors.
docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --target wasm32-unknown-unknown

# Serve the showcase at http://127.0.0.1:8180 (requires Trunk).
serve:
    cd shadcn-showcase && NO_COLOR=true trunk serve

# Build the production showcase into shadcn-showcase/dist-release (requires Trunk).
showcase-build:
    cd shadcn-showcase && NO_COLOR=true trunk build --release --dist dist-release

# Serve the existing production build at http://127.0.0.1:8181.
showcase-preview:
    python3 scripts/serve_showcase.py

# Exercise all showcase routes and key controls (start showcase-preview first).
showcase-smoke:
    python3 scripts/showcase_smoke.py

# Run the checks performed by CI, including the production showcase build.
ci: fmt-check check lint test docs showcase-build

# Print component coverage and test inventory (requires Python 3).
audit:
    python3 scripts/component_audit.py

# Regenerate icons from upstream Lucide SVGs (requires Python 3 and network access).
generate-icons:
    python3 scripts/generate_icons.py

# Remove Cargo build artifacts.
clean:
    cargo clean
