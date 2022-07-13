#!/bin/bash
set -ev

wasm-pack build --target web --release &&\

http-server -v -p 4000
