#!/bin/bash
set -e

echo "Building Rust binary..."
docker-compose -f docker-compose.build.yaml run --rm builder

echo "Stopping old containers..."
docker compose down

echo "Building Docker image..."
docker compose build

echo "Starting containers..."
docker compose up -d

echo "Init db..."
docker exec -it db bash /docker-entrypoint-initdb.d/init_db.sh

echo "Done!"