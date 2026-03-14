CREATE TABLE auth_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL,

    token TEXT UNIQUE NOT NULL,

    expires_at TIMESTAMP NOT NULL,

    created_by UUID,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_session_user
        FOREIGN KEY (user_id) REFERENCES users(id)
);
