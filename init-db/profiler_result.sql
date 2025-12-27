USE tempdb;
GO

IF OBJECT_ID('dbo.ProfilerResults', 'U') IS NULL
CREATE TABLE dbo.ProfilerResults (
    EventID INT IDENTITY(1,1) PRIMARY KEY,
    EventName NVARCHAR(50),
    SQLText NVARCHAR(MAX),
    CPU BIGINT,
    Duration BIGINT,
    Reads BIGINT,
    Writes BIGINT,
    ScanCount BIGINT,
    PhysicalReads BIGINT,
    ReadAheadReads BIGINT,
    LOBLogicalReads BIGINT,
    LOBPhysicalReads BIGINT,
    LOBReadAheadReads BIGINT,
    StartTime DATETIME2 DEFAULT SYSUTCDATETIME()
);
GO
