-- Your SQL goes here
CREATE TABLE api_logs(
    logs_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    trace_id TEXT NOT NULL,
    func_call TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    status TEXT NOT NULL,
    location TEXT,
    error_message TEXT
)
