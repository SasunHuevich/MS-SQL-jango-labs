#!/bin/bash
set -e

#echo "Building Rust binary..."
#docker-compose -f docker-compose.build.yaml run --rm builder

echo "Stopping old containers..."
docker compose down

echo "Building Docker image..."
docker compose build

echo "Starting db container..."
docker compose up -d db

echo "Sleep 5..."
sleep 5

echo "Init db..."
docker exec -it db bash /docker-entrypoint-initdb.d/init_db.sh

echo "Init query_profiler..."
docker exec -it db bash /docker-entrypoint-initdb.d/query_profile.sh

echo "Starting back container..."
docker compose up -d back

echo "Create profiler table..."
docker exec -it db bash /docker-entrypoint-initdb.d/profiler_table_create.sh

echo "Updating profiler table..."
docker exec -it db bash /docker-entrypoint-initdb.d/profiler_table_update.sh

echo "Read slowly requests..."
docker exec -it db bash /docker-entrypoint-initdb.d/analyze.sh

echo "Done!"