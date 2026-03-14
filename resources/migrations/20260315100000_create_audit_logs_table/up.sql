-- Create audit_logs table
CREATE TABLE audit_logs
(
    id            UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    user_id       UUID         NOT NULL,
    action        VARCHAR(255) NOT NULL,
    target_entity VARCHAR(255) NOT NULL,
    target_id     VARCHAR(255) NOT NULL,
    changes       JSONB,
    created_at    TIMESTAMP    NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMP    NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Create indexes for faster lookups
CREATE INDEX idx_audit_logs_user_id ON audit_logs (user_id);
CREATE INDEX idx_audit_logs_target ON audit_logs (target_entity, target_id);
