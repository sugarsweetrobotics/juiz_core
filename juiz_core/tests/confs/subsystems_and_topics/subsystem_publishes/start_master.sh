#!/bin/bash

# export RUST_LOG=juiz_core::topics=trace,juiz_core::core::core_broker=trace
cargo run -- -f tests/confs/subsystems_and_topics/subsystem_publishes/increment_process_master.conf -d