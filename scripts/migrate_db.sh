#! /bin/bash
set -e
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Install with: cargo install sqlx-cli --no-default-features --features native-tls,postgres"
    exit 1
fi

DB_USER="${DB_USER:=postgres}"
DB_PASSWORD="${DB_PASSWORD:=password}"
DB_NAME="${DB_NAME:=play_auth}"
DB_PORT="${DB_PORT:=5432}"
DB_HOST="${DB_HOST:=localhost}"

>&2 echo "Postgres is up and running on port ${DB_PORT} ${DB_NAME} - running migrations now"
>&2 echo "the name as it is ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||${DATABASE_URL}||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
# export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
# export DATABASE_URL="postgres://postgres:password@localhost:5432/play_auth"
# sqlx database create
>&2 echo "the name as it is ${DATABASE_URL}"
sqlx migrate --source=./auth run

# >&2 echo "Postgres has been migrated ready to go!"
# cargo watch -x run -p auth