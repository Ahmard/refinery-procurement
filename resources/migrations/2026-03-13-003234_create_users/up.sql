CREATE TABLE users
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    name          TEXT UNIQUE NOT NULL,
    email         TEXT UNIQUE NOT NULL,
    password_hash TEXT        NOT NULL,

    role          VARCHAR     NOT NULL,
    status        VARCHAR     NOT NULL DEFAULT 'ACTIVE',

    created_by    UUID        NULL,

    created_at    TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP   NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMP   NULL,

    CONSTRAINT fk_users_created_by
        FOREIGN KEY (created_by) REFERENCES users (id)
);

INSERT INTO users (id, name, email, password_hash, role, status)
VALUES ('2a78f742-465e-465c-a4dc-1d9678891024', 'superadmin', 'superadmin@ahmard.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ',
        'SUPERADMIN', 'ACTIVE'),
       ('4bc4d6c1-67d1-444e-9b94-7370376d35ac', 'supplier', 'supplier@ahmard.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ',
        'SUPPLIER', 'ACTIVE'),
       ('43988254-b494-44c5-9df9-c377ba275a19', 'user', 'user@ahmard.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ',
        'USER', 'ACTIVE');