echo "Create profiler table..."
docker exec -it db bash /docker-entrypoint-initdb.d/profiler_table_create.sh

echo "Updating profiler table..."
docker exec -it db bash /docker-entrypoint-initdb.d/profiler_table_update.sh

echo "Read slowly requests..."
docker exec -it db bash /docker-entrypoint-initdb.d/analyze.sh