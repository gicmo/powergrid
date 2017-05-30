#!/bin/sh

echo "Running in $PWD"
echo "DB is at: $DATABASE"
if [ ! -f "$DATABASE" ]; then
  echo "Initializing $DATABASE"
  flask setupdb
  cargo run -- setupdb --database "$DATABASE"
fi

exec cargo run -- --database "$DATABASE"
