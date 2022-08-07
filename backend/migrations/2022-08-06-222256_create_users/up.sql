CREATE TABLE users (
    id VARCHAR NOT NULL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    encrypted_password VARCHAR NOT NULL,
    salt VARCHAR NOT NULL
);

INSERT INTO users(id, email, encrypted_password, salt) VALUES ('86439bc8-dd22-4f78-bfd9-8ea902742886', 'test@test.com', 'BQYK3JiigHpWmzzbMndCIeDVSbkpg/73Q3eoa1EqSdw=', 'CVc+6gOEgHog2TBwdFPQvS5nbAp3TIuleVrUhQqz/Hs=');
INSERT INTO users(id, email, encrypted_password, salt) VALUES ('cdf36507-1135-4369-97ca-be1f367198b8', 'admin@admin.com', 'HvqiL2h8gEFaWqHMeARlubX3KJJ/i2lTmlXii1MnDAk=', 'FqU7bkDMEtvqCmPbs0tNA0dFbGING0SJAiMPq1jiWEM=');

