-- name: drop-table-person
DROP TABLE IF EXISTS "person";

-- name: create-table-person
CREATE TABLE "person" (id SERIAL PRIMARY KEY, name  VARCHAR NOT NULL, data BYTEA);