#!/bin/sh

#indexer init
#indexer run
cargo run -- init
cargo run -- run

exec "$@"
