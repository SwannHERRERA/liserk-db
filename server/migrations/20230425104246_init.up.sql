-- Add up migration script here
CREATE TABLE USERS (
  id_user UUID PRIMARY KEY,
  name VARCHAR(255),
  email VARCHAR(255)
);

CREATE TYPE PROCESS_STATUS AS ENUM ('active', 'cancel', 'crash', 'delete');

CREATE TABLE PROCESS (
  id_process UUID PRIMARY KEY,
  status VARCHAR(255) NOT NULL DEFAULT 'active',
  activation_date TIMESTAMP,
  creation_date TIMESTAMP
);
