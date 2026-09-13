-- Your SQL goes here
ALTER TABLE api_logs
ALTER COLUMN logs_id SET DEFAULT gen_random_uuid();