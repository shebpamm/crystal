-- Add the new 'uuid' column with UUIDv4 default value
ALTER TABLE kideaccounts
ADD COLUMN uuid UUID DEFAULT uuid_generate_v4();

-- Update the 'uuid' column with UUIDv4 values for existing rows
UPDATE kideaccounts
SET uuid = uuid_generate_v4()
WHERE uuid IS NULL;

-- Add a unique index to the 'uuid' column to ensure uniqueness
CREATE UNIQUE INDEX uuid_unique_index
ON kideaccounts (uuid);
