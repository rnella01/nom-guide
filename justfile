work:
    cargo watch -x "check --package nom-guide" -s "cargo test --package nom-guide"
test mod:
    cargo test --package nom-guide --lib -- {{mod}} --nocapture
lint:
    cargo clippy --package nom-guide
