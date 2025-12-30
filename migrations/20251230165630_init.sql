-- Add migration script here

CREATE TABLE peers (
    GUID BLOB PRIMARY KEY NOT NULL,
    id VARCHAR(100) NOT NULL,
    uuid blob NOT NULL,
    pk blob NOT NULL,
    created_at datetime not null DEFAULT(CURRENT_TIMESTAMP),
    user blob,
    status INTEGER,    
);

CREATE TABLE peer_connections (

);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
);

CREATE TABLE user_roles(
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);