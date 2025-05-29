#!/bin/bash
rustc "$1.rs" -o run
./run
rm run