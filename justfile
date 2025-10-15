test:
    cargo test -- --nocapture

lint:
    cargo clippy

format:
    cargo fmt --check

ci: test lint format
