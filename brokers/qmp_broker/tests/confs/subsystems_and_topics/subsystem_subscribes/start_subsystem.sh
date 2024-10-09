#!/bin/bash


export RUST_LOG=juiz_core::topics=trace,juiz_core=trace,qmp_broker=trace
cargo run -- -f brokers/qmp_broker/tests/confs/subsystems_and_topics/subsystem_subscribes/increment_process_subsystem.conf -d