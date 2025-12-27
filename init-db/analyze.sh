#!/bin/bash
set -e

/opt/mssql-tools18/bin/sqlcmd \
  -S localhost \
  -U sa \
  -P "$MSSQL_SA_PASSWORD" \
  -i /docker-entrypoint-initdb.d/analyze.sql \
  -C
