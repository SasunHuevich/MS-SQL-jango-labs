USE tempdb;
GO

-- Устанавливаем правильные SET-параметры для работы с XML
SET QUOTED_IDENTIFIER ON;
SET ANSI_NULLS ON;
GO

-- Выгрузка данных из Extended Events в таблицу
EXEC dbo.LoadExtendedEventsToProfilerResults;
GO

-- Анализ топ-10 медленных запросов
WITH TopQueries AS (
    SELECT TOP 10
        SQLText,
        AVG(Duration) AS AvgDuration_ms,
        MAX(Duration) AS MaxDuration_ms,
        SUM(CPU) AS TotalCPU,
        SUM(Reads) AS TotalReads,
        SUM(Writes) AS TotalWrites
    FROM dbo.ProfilerResults
    GROUP BY SQLText
    ORDER BY MaxDuration_ms DESC
)
SELECT
    q.SQLText,
    v.Metric,
    v.Value
FROM TopQueries q
CROSS APPLY (VALUES
    ('AvgDuration_ms', CAST(q.AvgDuration_ms AS varchar(50))),
    ('MaxDuration_ms', CAST(q.MaxDuration_ms AS varchar(50))),
    ('TotalCPU',       CAST(q.TotalCPU AS varchar(50))),
    ('TotalReads',     CAST(q.TotalReads AS varchar(50))),
    ('TotalWrites',    CAST(q.TotalWrites AS varchar(50)))
) v(Metric, Value)
ORDER BY q.MaxDuration_ms DESC, v.Metric;
GO