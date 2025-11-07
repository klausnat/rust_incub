-- Remove the check constraint first
ALTER TABLE roles 
DROP CONSTRAINT IF EXISTS valid_permissions;

-- Convert back to JSONB (you might need to handle data conversion)
ALTER TABLE roles 
ALTER COLUMN permissions TYPE JSONB USING permissions::JSONB;