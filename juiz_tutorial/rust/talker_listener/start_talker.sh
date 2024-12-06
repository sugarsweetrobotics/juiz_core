#!/bin/bash

export PWD=`pwd`
export DYLIB=$PWD/target/debug/librust_talker.dylib 
cd ../../../
export RUST_LOG=juiz_core::brokers::http=trace,juiz_core=debug,juiz_core::brokers::core_broker=trace

cargo run -p juiz_app  -- --process $DYLIB -1 -r 1.0 -d

#export PATH=$PATH:~/.cargo/bin
#juiz --process ./target/debug/librust_talker.dylib -1 -r 1.0 