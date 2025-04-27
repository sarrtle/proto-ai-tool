#!/bin/bash
set -e
cargo build --release --lib --target x86_64-unknown-linux-gnu
cargo build --release --lib --target x86_64-pc-windows-gnu

# Linux .so file (rename it properly)
cp target/x86_64-unknown-linux-gnu/release/libproto_ai.so ~/.config/nvim/lua/tools/proto_ai/proto_ai.so

# Windows .dll file
cp target/x86_64-pc-windows-gnu/release/proto_ai.dll ~/.config/nvim/lua/tools/proto_ai/proto_ai.dll
