CREATE TABLE catalog_item_compatibility (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    item_id UUID NOT NULL,
    compatible_item_id UUID NOT NULL,

    created_by UUID NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at     TIMESTAMP   NULL,

    UNIQUE (item_id, compatible_item_id),

    CONSTRAINT fk_compat_item
        FOREIGN KEY (item_id) REFERENCES catalog_items(id),

    CONSTRAINT fk_compat_compatible
        FOREIGN KEY (compatible_item_id) REFERENCES catalog_items(id),

    CONSTRAINT fk_compat_created_by
        FOREIGN KEY (created_by) REFERENCES users(id)
);
