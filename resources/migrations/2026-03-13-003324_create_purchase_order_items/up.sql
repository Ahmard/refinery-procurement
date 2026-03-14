CREATE TABLE purchase_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    purchase_order_id UUID NOT NULL,
    catalog_item_id UUID NOT NULL,

    quantity INTEGER NOT NULL,

    snapshot_price NUMERIC NOT NULL,
    snapshot_lead_time INTEGER,

    created_by UUID NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_poi_order
        FOREIGN KEY (purchase_order_id) REFERENCES purchase_orders(id),

    CONSTRAINT fk_poi_item
        FOREIGN KEY (catalog_item_id) REFERENCES catalog_items(id),

    CONSTRAINT fk_poi_created_by
        FOREIGN KEY (created_by) REFERENCES users(id)
);
