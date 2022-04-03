#!/usr/bin/env bash
set -x
set -eo pipefail


if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Install with: cargo install sqlx-cli --no-default-features --features native-tls,postgres"
    exit 1
fi

DB_USER="${DB_USER:=postgres}"
DB_PASSWORD="${DB_PASSWORD:=password}"
DB_NAME="${DB_NAME:=authey}"
DB_PORT="${DB_PORT:=5432}"
DB_HOST="${DB_HOST:=localhost}"

until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated ready to go!"