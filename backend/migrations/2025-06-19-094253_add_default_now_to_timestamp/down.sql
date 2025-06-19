-- This file should undo anything in `up.sql`
ALTER TABLE api_logs
RENAME COLUMN created_at TO "timestamp";

ALTER TABLE api_logs
ALTER COLUMN "timestamp" DROP DEFAULT;