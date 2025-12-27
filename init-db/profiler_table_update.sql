USE tempdb;
GO
SET ANSI_NULLS ON;
SET QUOTED_IDENTIFIER ON;
GO

CREATE OR ALTER PROCEDURE dbo.LoadExtendedEventsToProfilerResults
AS
BEGIN
    SET NOCOUNT ON;

    DECLARE @xml XML;

    SELECT @xml = CAST(t.target_data AS XML)
    FROM sys.dm_xe_sessions s
    JOIN sys.dm_xe_session_targets t
        ON s.address = t.event_session_address
    WHERE s.name = 'QueryProfiler'
      AND t.target_name = 'ring_buffer';

    IF @xml IS NULL
        RETURN;

    INSERT INTO dbo.ProfilerResults
    (
        EventName,
        SQLText,
        CPU,
        Duration,
        Reads,
        Writes,
        StartTime
    )
    SELECT
        evt.value('@name', 'nvarchar(50)'),
        evt.value('(action[@name="sql_text"]/value)[1]', 'nvarchar(max)'),
        evt.value('(data[@name="cpu_time"]/value)[1]', 'bigint'),
        evt.value('(data[@name="duration"]/value)[1]', 'bigint'),
        evt.value('(data[@name="logical_reads"]/value)[1]', 'bigint'),
        evt.value('(data[@name="writes"]/value)[1]', 'bigint'),
        evt.value('@timestamp', 'datetime2')
    FROM @xml.nodes('//event') AS X(evt);
END;
GO
