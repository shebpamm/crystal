-- Drop the unique index on the 'uuid' column
DROP INDEX IF EXISTS uuid_unique_index;

-- Remove the 'uuid' column
ALTER TABLE kideaccounts
DROP COLUMN IF EXISTS uuid;
