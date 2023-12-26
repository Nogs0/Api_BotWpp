-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE 
    IF NOT EXISTS drivers (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(50) NOT NULL,
        phone_number VARCHAR(20) NOT NULL,
        online BOOLEAN NOT NULL
    )