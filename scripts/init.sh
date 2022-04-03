#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$DB_USERNAME" --dbname "$DB_NAME" <<-EOSQL
    CREATE USER docker;
    CREATE DATABASE docker;
    GRANT ALL PRIVILEGES ON DATABASE docker TO docker;
EOSQL