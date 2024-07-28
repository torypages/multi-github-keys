#!/bin/bash
cargo build --release
ln -s $(pwd)/target/release/multi-github-keys "$HOME/.local/bin/mutli-github-keys"
