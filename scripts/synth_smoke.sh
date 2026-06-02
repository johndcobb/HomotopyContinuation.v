#!/bin/sh
set -eu

cd "$(dirname "$0")/.."
mkdir -p build

yosys -p 'synth_ice40 -top homotopy_Top -json build/homotopy.json' Verilog/source/*.v
nextpnr-ice40 --hx1k --package vq100 --json build/homotopy.json \
  --pcf Verilog/source/ice40h1xk.pcf --freq 25.00 --asc build/homotopy.asc
icepack build/homotopy.asc build/homotopy.bin
