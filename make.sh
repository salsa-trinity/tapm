#!/bin/bash

cargo build -q
mv ./target/debug/tapm ~/.cargo/bin/tapm
