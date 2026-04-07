-- Baseline migration: creates all tables in their final state.
-- Uses IF NOT EXISTS / ADD COLUMN IF NOT EXISTS so this is safe to run
-- against a database that was previously migrated by the TypeScript/Knex service.

CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY NOT NULL,
    email VARCHAR(400) NOT NULL,
    name VARCHAR(400),
    picture VARCHAR(4000),
    picture_image_service_id UUID,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS oauth_profile (
    sub VARCHAR PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES "user"(id),
    profile_json JSONB NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS access_request (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES "user"(id),
    key VARCHAR(400) NOT NULL,
    decided_at TIMESTAMP,
    granted BOOLEAN,
    granted_from TIMESTAMP,
    granted_until TIMESTAMP,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_terms_acceptance (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    terms_version VARCHAR(20) NOT NULL,
    accepted_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
