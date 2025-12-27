#!/bin/bash
if [ -z "$SA_PASSWORD" ]; then
    echo "SA_PASSWORD не задан!"
    exit 1
fi

docker exec -i db /opt/mssql-tools/bin/sqlcmd \
    -S localhost -U sa -P "$SA_PASSWORD" \
    -i /docker-entrypoint-initdb.d/query_profiler.sql

echo "Profiler запущен!"
