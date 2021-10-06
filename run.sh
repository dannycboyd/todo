#!/bin/bash
bin=$1;
if [ -z "$bin" ];
then
  echo Usage: 'run.sh [cli|service]\nbuild and run either todo_service or todo_cli';
fi
echo building and running ${bin:-service (default)};
cargo run --bin todo_${bin:-service};
