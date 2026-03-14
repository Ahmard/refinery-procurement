CREATE TABLE purchase_order_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    purchase_order_id UUID NOT NULL,

    status VARCHAR NOT NULL,

    created_by UUID NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_po_status_order
        FOREIGN KEY (purchase_order_id) REFERENCES purchase_orders(id),

    CONSTRAINT fk_po_status_created_by
        FOREIGN KEY (created_by) REFERENCES users(id)
);
