-- Add alcohol product support

-- Create alcohol type enum
CREATE TYPE alcohol_type AS ENUM ('cider', 'beer', 'spirits', 'wine');

-- Core alcohol product information
CREATE TABLE alcohol_product (
    item_name TEXT PRIMARY KEY,
    alcohol_type alcohol_type NOT NULL,
    volume_cl REAL NOT NULL CHECK (volume_cl > 0),
    supplier TEXT NOT NULL,
    FOREIGN KEY (item_name) REFERENCES item(name) ON DELETE CASCADE,
    FOREIGN KEY (supplier) REFERENCES supplier(name) ON DELETE RESTRICT
);

-- Current alcohol inventory state
CREATE TABLE alcohol_inventory (
    item_name TEXT PRIMARY KEY,
    current_bottles REAL NOT NULL CHECK (current_bottles >= 0),
    previous_bottles REAL NOT NULL CHECK (previous_bottles >= 0),
    current_purchase_price DECIMAL(10, 2) NOT NULL CHECK (current_purchase_price >= 0),
    previous_purchase_price DECIMAL(10, 2) CHECK (previous_purchase_price IS NULL OR previous_purchase_price >= 0),
    minimum_sale_price DECIMAL(10, 2) NOT NULL CHECK (minimum_sale_price >= 0),
    sale_price DECIMAL(10, 2) NOT NULL CHECK (sale_price >= 0),
    price_per_cl DECIMAL(10, 2) CHECK (price_per_cl IS NULL OR price_per_cl >= 0),
    last_updated TIMESTAMP WITH TIME ZONE,
    FOREIGN KEY (item_name) REFERENCES alcohol_product(item_name) ON DELETE CASCADE
);

-- Audit trail for alcohol inventory changes
CREATE TABLE alcohol_inventory_history (
    id SERIAL PRIMARY KEY,
    item_name TEXT NOT NULL,
    current_bottles REAL NOT NULL,
    previous_bottles REAL NOT NULL,
    current_purchase_price DECIMAL(10, 2) NOT NULL,
    previous_purchase_price DECIMAL(10, 2),
    minimum_sale_price DECIMAL(10, 2) NOT NULL,
    sale_price DECIMAL(10, 2) NOT NULL,
    price_per_cl DECIMAL(10, 2),
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_ TEXT NOT NULL,
    FOREIGN KEY (item_name) REFERENCES alcohol_product(item_name) ON DELETE CASCADE
);

-- Index for fast lookups
CREATE INDEX idx_alcohol_product_supplier ON alcohol_product(supplier);
CREATE INDEX idx_alcohol_inventory_history_item ON alcohol_inventory_history(item_name);
CREATE INDEX idx_alcohol_inventory_history_date ON alcohol_inventory_history(recorded_at);
