#!/usr/bin/env bash

wasm-pack build

cd pkg/
npm link
cd ../www
yarn # npm install
yarn run build # npm run build
npm link rustfractals