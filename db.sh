docker exec -it db /opt/mssql-tools18/bin/sqlcmd -S localhost -U sa -P 'StrongPass!123' -C

SELECT *
FROM dbo.YourTable
ORDER BY id  -- обязательно указывать ORDER BY
OFFSET 0 ROWS
FETCH NEXT 1000 ROWS ONLY;

SELECT TOP 1000 *
FROM dbo.YourTable;