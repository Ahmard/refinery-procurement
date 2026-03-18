ALTER TABLE purchase_orders
    ADD COLUMN total_cost NUMERIC(20, 4) NOT NULL DEFAULT 0;

ALTER TABLE purchase_order_items
    ALTER COLUMN quantity TYPE NUMERIC;
