-- Your SQL goes here
CREATE TABLE api_logs(
    logs_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trace_id TEXT NOT NULL,
    func_call TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    status TEXT NOT NULL,
    location TEXT,
    error_message TEXT
)
