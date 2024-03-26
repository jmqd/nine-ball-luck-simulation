#!/usr/bin/env sh

cargo run --release
./script/gen-apa-plot.sh
./script/gen-set-match-plot.sh
