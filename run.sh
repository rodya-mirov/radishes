#!/bin/bash

set -e

wasm-pack build --release --target web --out-name wasm --out-dir ./static

cd ./static

python3 -m http.server
