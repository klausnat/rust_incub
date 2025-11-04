-- Remove permissions column entirely
CREATE TABLE roles (
    slug VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    permissions VARCHAR NOT NULL
);

INSERT INTO roles (slug, name, permissions) VALUES
    ('admin', 'Administrator', 'all'),
    ('user', 'Regular User', 'read, write'), 
    ('viewer', 'Viewer', 'read');