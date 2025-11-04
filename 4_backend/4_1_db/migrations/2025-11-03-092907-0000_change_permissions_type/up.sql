-- Step 1: Create a temporary column to store the converted data
ALTER TABLE roles ADD COLUMN permissions_temp VARCHAR;

-- Step 2: Convert JSONB to text and set all to 'empty' for now
UPDATE roles SET permissions_temp = 'empty';

-- Step 3: Drop the old JSONB column
ALTER TABLE roles DROP COLUMN permissions;

-- Step 4: Rename the temporary column to permissions
ALTER TABLE roles RENAME COLUMN permissions_temp TO permissions;

-- Step 5: Add the check constraint
ALTER TABLE roles 
ADD CONSTRAINT valid_permissions 
CHECK (permissions IN ('full', 'read only', 'read and write', 'empty'));