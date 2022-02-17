-- This file should undo anything in `up.sql`
ALTER TABLE items
    DROP CONSTRAINT items_child_order_unique,
    DROP COLUMN child_order;