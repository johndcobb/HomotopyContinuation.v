#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

cargo run --manifest-path homotopycontinuationdriver/Cargo.toml -- \
  vectors homotopy_step --output Verilog/sim/vectors/homotopy_step_vectors.mem

verilator --binary -j 0 -Wall -Wno-fatal --trace-fst \
  --top-module Homotopy_Step_TB \
  Verilog/sim/Homotopy_Step_TB.sv Verilog/source/*.v

./obj_dir/VHomotopy_Step_TB
