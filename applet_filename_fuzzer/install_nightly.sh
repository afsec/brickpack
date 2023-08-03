#!/bin/sh

rustup install nightly

# cargo +nightly build
cd ./applet_filename_fuzzer
rustup override set nightly
