#!/bin/bash
set -ev

wasm-pack build --target web --release &&\

http-server -p 4000
