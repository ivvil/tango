-- Add migration script here

CREATE TABLE peers (
    peer_id VARCHAR(100) NOT NULL PRIMARY KEY,    
	address VARCHAR(50) NOT NULL,
	uuid BYTEA NOT NULL UNIQUE
);
