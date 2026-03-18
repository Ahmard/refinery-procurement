ALTER TABLE purchase_orders DROP COLUMN total_cost;

ALTER TABLE purchase_order_items
    ALTER COLUMN quantity TYPE INTEGER;
