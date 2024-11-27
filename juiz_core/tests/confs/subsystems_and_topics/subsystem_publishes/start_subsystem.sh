#!/bin/bash


#export RUST_LOG=juiz_core::topics=info,juiz_core=debug,juiz_core::brokers=trace,juiz_core::core=trace

cargo run -- -f tests/confs/subsystems_and_topics/subsystem_publishes/increment_process_subsystem.conf -d