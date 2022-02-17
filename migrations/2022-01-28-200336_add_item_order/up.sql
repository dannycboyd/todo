-- Your SQL goes here
ALTER TABLE items
    ADD child_order INTEGER NOT NULL DEFAULT 0;

UPDATE items i

with recursive children as (
    -- anchor, the static starting query
    select id, title, parent_id, row_number() over()
    from items
    where parent_id is null
    union
        select i.id, i.title, i.parent_id, row_number() over()
        from items i
        INNER JOIN children c ON c.id = i.parent_id
) UPDATE items i2
    SET child_order = c.row_number
    FROM children c
    WHERE i2.id = c.id;

-- since they must be unique and also not null, just set child_order in arbitrary ascending order
UPDATE items i
    set child_order = i2.seqnum
    FROM (select  i2.*, row_number() over() as seqnum from items i2) i2
    WHERE i.id = i2.id;

ALTER TABLE items
    ADD CONSTRAINT items_child_order_unique UNIQUE (parent_id, child_order);