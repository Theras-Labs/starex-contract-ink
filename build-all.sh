#!/usr/bin/env bash

set -eu

cargo contract build --manifest-path nft_asset/Cargo.toml
cargo contract build --manifest-path shop/Cargo.toml
cargo contract build
