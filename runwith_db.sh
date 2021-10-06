#!/bin/bash
bin=$1
sudo pg_ctlcluster 13 main start
if [ -z "$bin" ]
then
  echo Usage: 'runwith_db.sh [cli|service]\nbuild and run either todo_service (by default) or todo_cli'
fi
echo building and running ${bin:-service (default)}
cargo run --bin todo_${bin:-service};
