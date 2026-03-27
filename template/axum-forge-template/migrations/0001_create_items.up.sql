-- Migration: create_items
-- Create items table
CREATE TABLE IF NOT EXISTS items (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Optional seed data for development
INSERT INTO items (name)
VALUES
    ('First item'),
    ('Second item');