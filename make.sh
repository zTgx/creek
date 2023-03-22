cargo fmt --all && taplo fmt
cargo clippy -- -D warnings
cargo build --release