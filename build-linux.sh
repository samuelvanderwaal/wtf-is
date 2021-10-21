name=$(awk -F'[ ="]+' '$1 == "name" { print $2 }' Cargo.toml)
version=$(awk -F'[ ="]+' '$1 == "version" { print $2 }' Cargo.toml)
cargo build --release
cp ./target/release/$name ./binaries/$name-$version-x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/$name ./binaries/$name-$version-x86_64-unknown-linux-musl