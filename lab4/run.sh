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

echo "Starting django container..."
docker compose up -d django

echo "Done!"