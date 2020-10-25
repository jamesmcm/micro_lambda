#!/bin/sh
cargo build --release --target=x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./
strip --strip-all ./bootstrap
zip bootstrap.zip bootstrap
rm bootstrap
