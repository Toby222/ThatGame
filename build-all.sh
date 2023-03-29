set -e
set -x

# Useful for refreshing cache after changing dependencies

cargo build --target wasm32-unknown-unknown --release
cargo build --target wasm32-unknown-unknown
cargo build --target x86_64-unknown-linux-gnu --release
cargo build --target x86_64-unknown-linux-gnu