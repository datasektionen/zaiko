DROP VIEW next_inventory;

CREATE OR REPLACE FUNCTION next_inventory(i TEXT, c TEXT, s TEXT)
RETURNS TIMESTAMP WITH TIME ZONE
AS $$
SELECT
    MAX(log.time) + 
        LEAST(
            storage.inventory_interval,
            container.inventory_interval,
            item.inventory_interval
        ) as "time"
FROM log
JOIN item ON item.name = log.item
JOIN container ON container.name = log.container
JOIN storage ON storage.name = log.storage
WHERE
    log.item = i AND
    log.container = c AND
    log.storage = s
GROUP BY 
    storage.inventory_interval, 
    container.inventory_interval, 
    item.inventory_interval;
$$
LANGUAGE SQL;

ALTER TABLE item ADD CONSTRAINT item_inventory_interval_check CHECK (inventory_interval >= '1 day');
ALTER TABLE container ADD CONSTRAINT container_inventory_interval_check CHECK (inventory_interval >= '1 day');
ALTER TABLE storage ADD CONSTRAINT storage_inventory_interval_check CHECK (inventory_interval >= '1 day');
