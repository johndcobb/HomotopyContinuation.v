#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

cargo run --manifest-path homotopycontinuationdriver/Cargo.toml -- \
  vectors cubic_demo --output Verilog/sim/vectors/homotopy_core_vectors.mem

verilator --binary -j 0 -Wall -Wno-fatal --trace-fst \
  --top-module Homotopy_Core_TB \
  Verilog/sim/Homotopy_Core_TB.sv Verilog/source/*.v

./obj_dir/VHomotopy_Core_TB
