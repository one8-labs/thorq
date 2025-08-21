## PostgreSQL

`Commands used`

- sudo -i -u postgres
- SELECT * FROM pg_catalog.pg_user;
- createuser -s postgres
- psql -U postgres
- ALTER USER postgres WITH PASSWORD 'your_secure_password';
- psql -d DATABASE_NAME -U USERNAME
- \d (describe complete table)
- \d TABLE_NAME (describe a table)



`sqlx`

- cargo install sqlx-cli