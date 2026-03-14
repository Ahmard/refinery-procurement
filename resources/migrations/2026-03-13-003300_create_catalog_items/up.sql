CREATE TABLE catalog_items
(
    id             UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    secondary_id   TEXT UNIQUE NOT NULL,

    name           TEXT        NOT NULL,
    category       VARCHAR     NOT NULL,

    supplier_id    UUID        NOT NULL,

    manufacturer   TEXT,
    model          TEXT,

    price_usd      NUMERIC     NOT NULL,
    lead_time_days INTEGER,

    in_stock       BOOLEAN              DEFAULT TRUE,

    specs          JSONB,

    created_by     UUID        NOT NULL,

    created_at     TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMP   NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMP   NULL,

    CONSTRAINT fk_catalog_supplier
        FOREIGN KEY (supplier_id) REFERENCES suppliers (id),

    CONSTRAINT fk_catalog_created_by
        FOREIGN KEY (created_by) REFERENCES users (id)
);
