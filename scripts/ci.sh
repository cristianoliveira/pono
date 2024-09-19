#!/usr/bin/env bash

set -ex

VERSION="v$(cat Cargo.toml | grep version | awk -F\" '{print $2}')"

# Add macOS Rust target
rustup target add $TARGET

cargo build --target $TARGET --release

ARTIFACT="pono-${RELEASE_TAG:-"$VERSION"}-${TARGET}.tar.gz"

mkdir -p pkg
cp target/$TARGET/release/pono pkg

tar czf "$ARTIFACT" pkg

# sanity check the file type
file pkg/pono

