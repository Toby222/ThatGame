set -e
set -x

# Useful for refreshing cache after changing dependencies

cargo build --verbose --target wasm32-unknown-unknown
cargo build --verbose --target wasm32-unknown-unknown --release
cargo build --verbose --target x86_64-unknown-linux-gnu
cargo build --verbose --target x86_64-unknown-linux-gnu --release