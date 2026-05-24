#!/bin/bash
set -e

echo "Stopping old containers..."
docker compose down

echo "Building Docker image..."
docker compose build db django

echo "Starting db container..."
docker compose up -d db

echo "Sleep 15..."
sleep 15

echo "Init db..."
docker exec db bash /docker-entrypoint-initdb.d/init_db.sh

echo "Starting django container..."
docker compose up -d django

echo "Sleep 5..."
sleep 5

echo "Migrate..."
docker compose exec django python manage.py migrate --noinput

echo "Done!"