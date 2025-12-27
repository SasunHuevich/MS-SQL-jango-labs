docker exec -it db /opt/mssql-tools18/bin/sqlcmd -S localhost -U sa -P 'StrongPass!123' -C

SELECT *
FROM dbo.YourTable
ORDER BY id  -- обязательно указывать ORDER BY
OFFSET 0 ROWS
FETCH NEXT 1000 ROWS ONLY;

SELECT TOP 1000 *
FROM dbo.YourTable;

docker exec -it db ls -lh /var/opt/mssql/data/query_profiler.xel


SELECT TOP 10 
    SQLText,
    AVG(Duration) AS AvgDuration_ms,
    MAX(Duration) AS MaxDuration_ms,
    SUM(CPU) AS TotalCPU,
    SUM(Reads) AS TotalReads,
    SUM(Writes) AS TotalWrites
FROM tempdb.ProfilerResults
GROUP BY SQLText
ORDER BY MaxDuration_ms DESC;