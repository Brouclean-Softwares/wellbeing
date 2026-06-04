CREATE TABLE IF NOT EXISTS satisfying_moments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users ON DELETE CASCADE,
    description TEXT NOT NULL,
    thoughts TEXT,
    why_it_matters TEXT,
    values_alignment TEXT,
    lived_at DATE NOT NULL DEFAULT CURRENT_DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS satisfying_moments_relatives (
    moment_id BIGINT NOT NULL REFERENCES satisfying_moments ON DELETE CASCADE,
    relative_id BIGINT NOT NULL REFERENCES users ON DELETE CASCADE,
    viewed_at TIMESTAMPTZ
);