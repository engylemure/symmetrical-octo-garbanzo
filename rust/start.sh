#!/bin/bash

if [ "$RUST_ENV" == "dev" ]
then cargo run
else cargo run --release
fi