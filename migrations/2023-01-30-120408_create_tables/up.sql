CREATE TABLE accounts (
    account_id SERIAL PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL
);

CREATE TABLE todos (
    account_id integer REFERENCES accounts (account_id),
    todo_id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL
);