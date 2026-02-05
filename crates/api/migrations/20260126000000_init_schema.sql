-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users Table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Upload Summaries Table
CREATE TABLE IF NOT EXISTS upload_summaries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    total_trips INTEGER NOT NULL DEFAULT 0,
    cost_actual DECIMAL(10, 2) NOT NULL DEFAULT 0.00,
    cost_optimized DECIMAL(10, 2) NOT NULL DEFAULT 0.00,
    savings DECIMAL(10, 2) NOT NULL DEFAULT 0.00,
    uploaded_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster history lookups
CREATE INDEX idx_upload_summaries_user_id ON upload_summaries(user_id);
