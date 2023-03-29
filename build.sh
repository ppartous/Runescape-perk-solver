#!/bin/sh
cargo build --release
cp ./target/release/perk_solver_cli ./perk_solver
cp ./target/release/libperk_solver.so ./perk_solver.so