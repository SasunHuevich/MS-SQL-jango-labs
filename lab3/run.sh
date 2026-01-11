#!/bin/bash
set -e

echo "Stopping old containers..."
docker compose down

echo "Building Docker image..."
docker compose build

echo "Starting db container..."
docker compose up -d db

echo "Sleep 15..."
sleep 15

# echo "Init query_profiler..."
# docker exec -it db bash /docker-entrypoint-initdb.d/query_profile.sh

echo "Starting back container..."
LAB_MODE=lab3 docker compose up -d back

echo "Done!"