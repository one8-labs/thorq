# Setup

- Make sure you have PostgreSQL installed
- create a .env file at the root consisting of `DATABASE_URL`
- Create database called thorq
- Create database tables by executing the [sql](/docs/sql/thorq.sql)
- You can make use of the job_ingestion script from the scripts folder to ingest the jobs data.
- Run the project `cargo run`
- Create queue
    ```
    curl -X POST http://localhost:3000/create-queue \
        -H "Content-Type: application/json" \
        -d '{
            "id": 1,
            "name": "my-queue",
            "concurrency": 10,
            "rate_per_sec": 100,
            "created_at": "2024-01-01T00:00:00Z"
        }'
    ```