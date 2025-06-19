-- Your SQL goes here
ALTER TABLE api_logs 
RENAME COLUMN "timestamp" TO created_at;

ALTER TABLE api_logs
ALTER COLUMN created_at SET DEFAULT now();
