#!/bin/bash

set -e

# build frontend assets and put them in a place the Rocket server
# expects

echo "building ui"
pushd tdv_ui
cargo web build --release --target=wasm32-unknown-unknown
popd
echo "ui build complete"

cp tdv_ui/target/wasm32-unknown-unknown/release/tdv_ui.js \
tdv_server/static/tdv_ui.js
cp tdv_ui/target/wasm32-unknown-unknown/release/tdv_ui.wasm \
tdv_server/static/tdv_ui.wasm
#cp ui/static/styles.css server/static/styles.css

(
  echo "running server"
  cd tdv_server
  cargo run --release
)
