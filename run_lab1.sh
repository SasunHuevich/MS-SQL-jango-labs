#!/bin/bash
set -e

echo "Stopping old containers..."
docker compose down

echo "Building Docker image..."
docker compose build

echo "Starting db container..."
docker compose up -d db

echo "Sleep 10..."
sleep 10

echo "Init db..."
docker exec -it db bash /docker-entrypoint-initdb.d/init_db.sh

echo "Starting back container..."
docker compose up -d back

echo "Done!"