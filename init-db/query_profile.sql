-- Удаляем сессию, если она существует
IF EXISTS (SELECT * FROM sys.server_event_sessions WHERE name = 'QueryProfiler')
BEGIN
    ALTER EVENT SESSION [QueryProfiler] ON SERVER STATE = STOP;
    DROP EVENT SESSION [QueryProfiler] ON SERVER;
END
GO

-- Создаем объединенную сессию Extended Events
CREATE EVENT SESSION [QueryProfiler] ON SERVER
-- Отслеживаем завершение отдельных SQL-запросов
ADD EVENT sqlserver.sql_statement_completed(
    ACTION(sqlserver.sql_text, sqlserver.client_app_name, sqlserver.username)
),
-- Отслеживаем завершение батчей
ADD EVENT sqlserver.sql_batch_completed(
    ACTION(sqlserver.sql_text, sqlserver.client_app_name, sqlserver.username)
),
-- Отслеживаем завершение RPC вызовов
ADD EVENT sqlserver.rpc_completed(
    ACTION(sqlserver.sql_text, sqlserver.client_app_name, sqlserver.username)
)
-- Настраиваем два таргета: файл и ring_buffer
ADD TARGET package0.event_file(
    SET filename=N'/var/opt/mssql/data/query_profiler.xel',
        max_file_size=(5), max_rollover_files=(5)
),
ADD TARGET package0.ring_buffer;
GO

-- Запускаем сессию
ALTER EVENT SESSION [QueryProfiler] ON SERVER STATE = START;
GO
-- Удаляем сессию, если она существует
IF EXISTS (SELECT * FROM sys.server_event_sessions WHERE name = 'QueryProfiler')
BEGIN
    ALTER EVENT SESSION [QueryProfiler] ON SERVER STATE = STOP;
    DROP EVENT SESSION [QueryProfiler] ON SERVER;
END
GO

-- Создаем объединенную сессию Extended Events
CREATE EVENT SESSION [QueryProfiler] ON SERVER
-- Отслеживаем завершение отдельных SQL-запросов
ADD EVENT sqlserver.sql_statement_completed(
    ACTION(sqlserver.sql_text, sqlserver.client_app_name, sqlserver.username)
),
-- Отслеживаем завершение батчей
ADD EVENT sqlserver.sql_batch_completed(
    ACTION(sqlserver.sql_text, sqlserver.client_app_name, sqlserver.username)
),
-- Отслеживаем завершение RPC вызовов
ADD EVENT sqlserver.rpc_completed(
    ACTION(sqlserver.sql_text, sqlserver.client_app_name, sqlserver.username)
)
-- Настраиваем два таргета: файл и ring_buffer
ADD TARGET package0.event_file(
    SET filename=N'/var/opt/mssql/data/query_profiler.xel',
        max_file_size=(5), max_rollover_files=(5)
),
ADD TARGET package0.ring_buffer;
GO

-- Запускаем сессию
ALTER EVENT SESSION [QueryProfiler] ON SERVER STATE = START;
GO
