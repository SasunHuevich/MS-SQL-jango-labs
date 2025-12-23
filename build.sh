#!/bin/bash
set -e

echo "Building Rust binary..."
docker-compose -f docker-compose.build.yaml run builder