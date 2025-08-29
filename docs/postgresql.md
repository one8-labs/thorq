## PostgreSQL

`Commands used`

- sudo -i -u postgres
- SELECT * FROM pg_catalog.pg_user;
- createuser -s postgres
- psql -U postgres
- ALTER USER postgres WITH PASSWORD 'your_secure_password';
- psql -d DATABASE_NAME -U USERNAME
- \d: describe complete table
- \d TABLE_NAME: describe a table
- \c DATABASE_NAME: Connect to database
- \! clear: Clears the screen


`PostgreSQL commands in macos`

- psql postgres
- CREATE ROLE postgres WITH LOGIN PASSWORD 'postgres';
- ALTER ROLE postgres WITH SUPERUSER;



`sqlx`

- cargo install sqlx-cli
- cargo sqlx prepare