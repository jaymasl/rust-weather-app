CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS weather_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    client_ip TEXT NOT NULL,
    user_agent TEXT,
    search_query TEXT NOT NULL,

    weather_data JSONB NOT NULL
);