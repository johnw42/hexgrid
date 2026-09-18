#! /bin/bash

export RUST_BACKTRACE=1
while
	cargo test
	cargo run --example demo
do
	sleep 1
done
