#!/bin/bash

cargo build --target wasm32-wasi --release
# wasmedgec target/wasm32-wasi/release/message_service.wasm message_service.wasm

dapr stop message-service
sudo dapr run --app-id message-service \
    --app-protocol http \
    --app-port 9007 \
    --dapr-http-port 3505 \
    --log-level debug \
    --resources-path ../config \
    -- wasmedge --dir .:. message_service.wasm
