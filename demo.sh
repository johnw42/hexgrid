#! /bin/bash

while cargo test && cargo run --example demo; do sleep 1; done
