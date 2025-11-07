CREATE TABLE users_roles (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_slug VARCHAR NOT NULL REFERENCES roles(slug) ON DELETE CASCADE,
    assigned_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_slug)
);

CREATE INDEX idx_users_roles_user_id ON users_roles(user_id);
CREATE INDEX idx_users_roles_role_slug ON users_roles(role_slug);