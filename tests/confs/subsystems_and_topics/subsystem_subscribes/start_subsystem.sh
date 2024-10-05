#!/bin/bash


# export RUST_LOG=juiz_core::topics=trace, juiz_core=trace

cargo run -- -f tests/confs/subsystems_and_topics/subsystem_subscribes/increment_process_subsystem.conf -d