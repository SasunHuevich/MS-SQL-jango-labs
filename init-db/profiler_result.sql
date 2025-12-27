USE tempdb;
GO

IF OBJECT_ID('dbo.ProfilerResults', 'U') IS NULL
CREATE TABLE dbo.ProfilerResults (
    EventID INT IDENTITY(1,1) PRIMARY KEY,
    EventName NVARCHAR(50),
    SQLText NVARCHAR(MAX),
    CPU INT,
    Duration BIGINT,
    Reads BIGINT,
    Writes BIGINT,
    StartTime DATETIME DEFAULT GETDATE()
);
GO
