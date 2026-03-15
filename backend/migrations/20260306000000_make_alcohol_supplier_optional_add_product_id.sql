ALTER TABLE alcohol_product
    ADD COLUMN IF NOT EXISTS product_id TEXT;

ALTER TABLE alcohol_product
    ALTER COLUMN supplier DROP NOT NULL;