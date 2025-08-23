CREATE TABLE queues (
    id            BIGSERIAL PRIMARY KEY,
    name          TEXT UNIQUE NOT NULL,
    concurrency   INT NOT NULL DEFAULT 10,       
    rate_per_sec  DOUBLE PRECISION,              
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE TABLE jobs (
    id               BIGSERIAL PRIMARY KEY,
    queue_id         BIGINT NOT NULL REFERENCES queues(id) ON DELETE RESTRICT,
    type             TEXT NOT NULL,                          
    payload          JSONB NOT NULL,                         
    priority         INT NOT NULL DEFAULT 0,                 
    status           TEXT NOT NULL DEFAULT 'pending',        
    run_at           TIMESTAMPTZ NOT NULL DEFAULT now(),     
    inserted_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    attempts         INT NOT NULL DEFAULT 0,
    max_attempts     INT NOT NULL DEFAULT 25,
    last_error       TEXT,
    timeout_sec      INT NOT NULL DEFAULT 300,               
    idempotency_key  TEXT,                                   
    tenant_id        TEXT,                                   
    UNIQUE (idempotency_key)                               
);


CREATE TABLE job_attempts (
    id           BIGSERIAL PRIMARY KEY,
    job_id       BIGINT NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    started_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at  TIMESTAMPTZ,
    success      BOOLEAN,
    error        TEXT
);