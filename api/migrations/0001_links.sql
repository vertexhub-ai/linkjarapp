CREATE TABLE IF NOT EXISTS links (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    code        TEXT        UNIQUE NOT NULL,
    url         TEXT        NOT NULL,
    hit_count   INT         NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
