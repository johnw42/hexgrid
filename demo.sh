#! /bin/bash

export RUST_BACKTRACE=1
ulimit -c unlimited
while
	cargo test
	cargo run --example demo
do
	sleep 1
done
