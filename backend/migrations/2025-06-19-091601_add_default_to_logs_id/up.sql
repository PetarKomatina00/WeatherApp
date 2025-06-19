-- Your SQL goes here
ALTER TABLE api_logs
ALTER COLUMN logs_id SET DEFAULT uuid_generate_v4();