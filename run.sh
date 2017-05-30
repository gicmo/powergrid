#!/bin/sh

echo "Running in $PWD"
echo "DB is at: $DATABASE"
if [ ! -f "$DATABASE" ]; then
  echo "Initializing $DATABASE"
  /srv/target/release/powergrid setupdb --database "$DATABASE"
fi

exec /srv/target/release/powergrid --database "$DATABASE"
