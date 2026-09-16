#! /bin/bash

export RUST_BACKTRACE=0
while cargo test && cargo run --example demo; do
	sleep 1
done
