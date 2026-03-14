CREATE TABLE purchase_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    po_number TEXT UNIQUE,

    supplier_id UUID NOT NULL,

    created_by UUID NOT NULL,

    requestor TEXT,
    cost_center TEXT,
    payment_terms TEXT,
    needed_by_date DATE,

    status VARCHAR NOT NULL DEFAULT 'Draft',

    idempotency_key TEXT UNIQUE,

    submitted_at TIMESTAMP,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_po_supplier
        FOREIGN KEY (supplier_id) REFERENCES suppliers(id),

    CONSTRAINT fk_po_created_by
        FOREIGN KEY (created_by) REFERENCES users(id)
);
