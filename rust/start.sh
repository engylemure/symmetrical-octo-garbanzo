#!/bin/bash

if [ "$RUST_ENV" == "dev" ]
then cargo watch -x run --release
else ./target/release/rust-server
fi