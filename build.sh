#!/bin/sh
# cargo build --release
# cp ./target/release/perk_solver ./perk_solver
cargo build --profile=release-with-debug
cp ./target/release-with-debug/perk_solver ./perk_solver