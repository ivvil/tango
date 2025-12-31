-- Add migration script here

CREATE TABLE peers (
    id UUID PRIMARY KEY,
    peer_id VARCHAR(100) NOT NULL,    
    uuid blob NOT NULL,
    pk blob NOT NULL,
    created_at datetime not null DEFAULT(CURRENT_TIMESTAMP),
    user blob,    
);

CREATE TABLE peer_status (
    device_id UUID PRIMARY KEY REFERENCES peers(id) ON DELETE CASCADE,
    last_seen TIMESTAMPTZ NOT NULL,
    status INTEGER,        
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_users_email ON users(email);

CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
);

CREATE UNIQUE INDEX idx_roles_name ON roles(name);

CREATE TABLE user_roles(
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

CREATE INDEX idx_user_roles_user ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role_id);

CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT false,
);

CREATE INDEX idx_sessions_id ON sessions(id);
CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_sessions_expires ON sessions(expires_at);