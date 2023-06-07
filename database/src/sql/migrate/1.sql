CREATE TABLE Users (
    id SERIAL,
    uuid VARCHAR,
    email VARCHAR,
    passhash VARCHAR
);

CREATE TABLE Tokens (
    id SERIAL,
    name VARCHAR,
    max_supply VARCHAR
);

CREATE TABLE Wallets (
    id SERIAL,
    addr VARCHAR
);

CREATE TABLE WalletTokens (
    id SERIAL,
    token_id INTEGER REFERENCES Tokens,
    wallet_id INTEGER REFERENCES Wallets,
    amount VARCHAR
);



