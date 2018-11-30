#!/usr/bin/env bash

wasm-pack build

cd pkg/
npm link
cd ../www
yarn
yarn run build
npm link rustfractals