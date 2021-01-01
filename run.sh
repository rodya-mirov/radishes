#!/bin/bash

set -e

mkdir -p ./pkg

wasm-pack build --release --target web --out-name wasm --out-dir ./pkg

# wasm-pack generates a lot of garbage we don't need and is annoying
# so we copy the two files we actually do need and ignore the rest
cp ./pkg/wasm.js ./static
cp ./pkg/wasm_bg.wasm ./static

cd ./static

python3 -m http.server
