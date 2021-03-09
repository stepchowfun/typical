#!/usr/bin/env bash
set -euo pipefail

# This script generates release artifacts in a directory called `release`. It should be run from a
# macOS machine with an x86-64 processor. Usage:
#   ./release.sh

# The release process involves three steps:
# 1. Bump the version in `Cargo.toml`, run `cargo build` to update `Cargo.lock`, and update
#    `CHANGELOG.md` with information about the new version. Ship those changes as a single pull
#    request.
# 2. Run this script on an x86-64 machine and upload the files in the `release` directory to GitHub
#    as release artifacts.
# 3. Update the version in `install.sh` to point to the new release.

# We wrap everything in parentheses to ensure that any working directory changes with `cd` are local
# to this script and don't affect the calling user's shell.
(
  # x86-64 macOS build
  rm -rf target/release
  cargo build --release

  # x86-64 GNU/Linux build
  rm -rf artifacts
  toast release

  # Prepare the `release` directory.
  rm -rf release
  mkdir release

  # Copy the artifacts into the `release` directory.
  cp target/release/typical release/typical-x86_64-apple-darwin
  cp artifacts/typical-x86_64-unknown-linux-gnu release/typical-x86_64-unknown-linux-gnu

  # Compute checksums of the artifacts.
  cd release
  shasum --algorithm 256 --binary typical-x86_64-apple-darwin > typical-x86_64-apple-darwin.sha256
  shasum --algorithm 256 --binary typical-x86_64-unknown-linux-gnu > typical-x86_64-unknown-linux-gnu.sha256

  # Verify the checksums.
  shasum --algorithm 256 --check --status typical-x86_64-apple-darwin.sha256
  shasum --algorithm 256 --check --status typical-x86_64-unknown-linux-gnu.sha256

  # Publish to crates.io.
  cargo publish
)
