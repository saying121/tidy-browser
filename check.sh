#!/usr/bin/env bash

cargo clippy --target x86_64-unknown-linux-gnu
cargo clippy --target aarch64-apple-darwin
cargo clippy --target x86_64-pc-windows-gnu
