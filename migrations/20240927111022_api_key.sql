CREATE TABLE af_api_key (
    id              SERIAL     PRIMARY KEY,
    workspace_id    UUID       REFERENCES af_workspace(workspace_id) ON DELETE CASCADE NOT NULL,
    uid             BIGINT     REFERENCES af_user(uid)               ON DELETE CASCADE NOT NULL,
    api_key_hash    BYTEA      NOT NULL,
    created_at      TIMESTAMP  DEFAULT NOW(),
    last_used       TIMESTAMP  DEFAULT NOW(),
    status          SMALLINT   NOT NULL DEFAULT 0, -- 0: active, 1: inactive
    scopes          SMALLINT[] NOT NULL,
    expiration_date TIMESTAMP  NOT NULL
);