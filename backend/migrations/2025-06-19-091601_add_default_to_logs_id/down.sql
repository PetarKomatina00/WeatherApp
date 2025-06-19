-- This file should undo anything in `up.sql`
ALTER TABLE api_logs
ALTER COLUMN logs_id DROP DEFAULT;