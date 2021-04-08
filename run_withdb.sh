#!/bin/bash
bin=$1
sudo pg_ctlcluster 13 main start
echo building and running ${bin}
cargo run --bin todo_${bin}