#!/usr/bin/env bash

# This script is used to build the artifacts for the project.
# for all the platforms and architectures in rust

set -e

# For each platform and architecture, build the artifacts, set TARGET and 
APPLE_TARGETS=(
  "x86_64-apple-darwin"
  # "aarch64-apple-darwin"
)
LINUX_TARGETS=(
  "x86_64-unknown-linux-gnu"
  # "aarch64-unknown-linux-gnu"
  # "arm-unknown-linux-gnueabihf"
  # "armv7-unknown-linux-gnueabihf"
  # "i686-unknown-linux-gnu"
)
WINDOWS_TARGETS=(
  "i686-pc-windows-gnu"
  # "i686-pc-windows-msvc"
  # "x86_64-pc-windows-gnu"
  # "x86_64-pc-windows-msvc"
)

LIBRARY_TARGETS=(
  "${APPLE_TARGETS[@]}"
  "${LINUX_TARGETS[@]}"
  "${WINDOWS_TARGETS[@]}"
)

for TARGET in "${LIBRARY_TARGETS[@]}"; do
  echo "Building for $TARGET"
  VERSION="v$(grep version Cargo.toml | awk -F\" '{print $2}')"

  # Add macOS Rust target
  rustup target add $TARGET

  cargo update 
  cargo build --target $TARGET --release

  ARTIFACT="pono-${RELEASE_TAG:-"$VERSION"}-${TARGET}.tar.gz"

  mkdir -p pkg
  cp target/$TARGET/release/pono pkg

  tar czf "$ARTIFACT" pkg

  # sanity check the file type
  file pkg/pono
done
