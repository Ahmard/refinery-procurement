CREATE TABLE suppliers
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id       UUID        NOT NULL,
    created_by    UUID        NOT NULL,

    name          TEXT        NOT NULL,
    contact_email TEXT,
    contact_phone TEXT,
    address       TEXT,

    status        VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',

    created_at    TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP   NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMP   NULL,

    CONSTRAINT fk_suppliers_created_by
        FOREIGN KEY (created_by) REFERENCES users (id)
);

INSERT INTO suppliers (id, user_id, created_by, name, contact_email, contact_phone, address, status)
VALUES ('b8830d15-2c52-4296-9708-34dceb2c39cb', '4bc4d6c1-67d1-444e-9b94-7370376d35ac',
        '2a78f742-465e-465c-a4dc-1d9678891024', 'Supplier',
        'supplier@ahmard.com', '+2347011223344', '2308 Jane Doe Street', 'ACTIVE');
