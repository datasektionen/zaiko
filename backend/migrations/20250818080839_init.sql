CREATE EXTENSION fuzzystrmatch;

CREATE TYPE state AS ENUM ('none', 'good', 'warning', 'critical', 'incoming');

CREATE TYPE storage_listing_basic AS (
    storage TEXT,
    container TEXT,
    state STATE
);

CREATE TYPE storage_listing AS (
    storage TEXT,
    container TEXT,
    amount REAL,
    min REAL,
    max REAL,
    state STATE,
    next_inventory TIMESTAMP WITH TIME ZONE
);

CREATE TYPE supplier_listing AS (
    name TEXT,
    link TEXT,
    prefered BOOL
);

CREATE TYPE shipment_listing AS (
    name TEXT,
    amout REAL
);

CREATE TABLE "storage" (
    name TEXT,
    protected bool NOT NULL DEFAULT FALSE,
    inventory_interval INTERVAL,
    PRIMARY KEY (name),
    CHECK (TRIM(name) <> '')
);

CREATE TABLE "container" (
    name TEXT,
    storage TEXT,
    inventory_interval INTERVAL,
    PRIMARY KEY (name, storage),
    FOREIGN KEY (storage) REFERENCES storage (name) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE "item" (
    name TEXT,
    inventory_interval INTERVAL,
    unit TEXT NOT NULL DEFAULT 'st',
    PRIMARY KEY (name),
    CHECK (TRIM(name) <> ''),
    CHECK (TRIM(unit) <> '')
);

CREATE TABLE "stored_item" (
    storage TEXT,
    container TEXT,
    item TEXT,
    min REAL,
    max REAL,
    amount REAL NOT NULL,
    PRIMARY KEY (storage, container, item),
    FOREIGN KEY (storage, container) REFERENCES container (storage, name) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (item) REFERENCES item (name) ON DELETE CASCADE ON UPDATE CASCADE,
    CHECK (amount >= 0),
    CHECK ((min IS NULL AND max IS NULL) OR (min IS NOT NULL AND max IS NOT NULL)),
    CHECK (min >= 0),
    CHECK (max > min)
);

CREATE TABLE "supplier" (
    name TEXT,
    notes TEXT,
    username TEXT,
    password TEXT,
    link TEXT,
    mandate TEXT NOT NULL,
    PRIMARY KEY (name),
    CHECK (TRIM(notes) <> ''),
    CHECK (TRIM(username) <> ''),
    CHECK (TRIM(password) <> ''),
    CHECK (TRIM(link) <> ''),
    CHECK (TRIM(mandate) <> '')
);

CREATE TABLE "supplier_item" (
    supplier TEXT,
    item TEXT,
    link TEXT,
    prefered bool NOT NULL DEFAULT FALSE,
    PRIMARY KEY (supplier, item),
    FOREIGN KEY (supplier) REFERENCES supplier (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (item) REFERENCES item (name) ON DELETE CASCADE ON UPDATE CASCADE,
    CHECK (TRIM(link) <> '')
);

CREATE TABLE "log" (
    item TEXT NOT NULL,
    storage TEXT NOT NULL,
    container TEXT NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    amount REAL NOT NULL,
    user_ TEXT NOT NULL,
    PRIMARY KEY (item, time),
    FOREIGN KEY (item, storage, container) REFERENCES stored_item (item, storage, container) ON DELETE CASCADE ON UPDATE CASCADE,
    CHECK (amount >= 0),
    CHECK (TRIM(user_) <> '')
);

CREATE TABLE "move_log" (
    item TEXT NOT NULL,
    from_storage TEXT NOT NULL,
    from_container TEXT NOT NULL,
    to_storage TEXT NOT NULL,
    to_container TEXT NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    amount REAL NOT NULL,
    user_ TEXT NOT NULL,
    PRIMARY KEY (item, time),
    FOREIGN KEY (item) REFERENCES item (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (to_storage, to_container) REFERENCES container (storage, name) ON UPDATE CASCADE ON DELETE CASCADE,
    CHECK (amount > 0),
    CHECK (TRIM(user_) <> '')
);

CREATE TABLE "shipment" (
    id UUID,
    time_created TIMESTAMP WITH TIME ZONE NOT NULL,
    time_arive TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE "shipment_item" (
    shipment UUID,
    item TEXT,
    amount REAL NOT NULL,
    PRIMARY KEY (shipment, item),
    FOREIGN KEY (shipment) REFERENCES shipment (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (item) REFERENCES item (name) ON DELETE CASCADE ON UPDATE CASCADE,
    CHECK (amount >= 0)
);

CREATE OR REPLACE FUNCTION state(amount REAL, min REAL, max REAL, incoming BOOL)
RETURNS STATE
AS 
$$
DECLARE
    warning_limit REAL = (max - min) * 0.1;
BEGIN
    IF incoming THEN
        RETURN 'incoming';
    ELSIF min IS NULL OR max IS NULL OR amount IS NULL THEN
        RETURN 'none';
    ELSIF amount < min THEN
        RETURN 'critical';
    ELSIF amount < min + warning_limit THEN
        RETURN 'warning';
    ELSE
        RETURN 'good';
    END IF;
END;
$$ 
language 'plpgsql';

CREATE VIEW avrage_consuption AS
SELECT item, amount - LAG(amount) OVER(ORDER BY time) AS change
FROM log
JOIN storage ON storage.name = log.storage
WHERE time < CURRENT_TIMESTAMP - interval '1 month';

