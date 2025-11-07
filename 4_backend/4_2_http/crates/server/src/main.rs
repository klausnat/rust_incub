use shared::{Command, Response};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::convert::Infallible;
use warp::Filter;

// Database connection pool type alias
type DbPool = Pool<Postgres>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Initialize database connection pool
    let database_url = "postgres://rust_user:rust_password@localhost/rust_cli_app";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Connected to database successfully!");

    // Step 2: Create Warp routes (create API endpoint that can handle JSON commands, with CORS)
    let api = warp::path("api");

    // CORS support for cross-origin requests
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_header("content-type");

    // Main command endpoint
    let command_route = api
        .and(warp::path("command"))
        .and(warp::post())
        .and(with_db(pool.clone()))
        .and(warp::body::json())
        .and_then(handle_command);

    // Combine all routes
    let routes = command_route.with(cors);

    println!("Server starting on http://localhost:3030");

    // Step 3: Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

// Database pool filter
fn with_db(pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// Main command handler - dispatches to specific handlers
async fn handle_command(pool: DbPool, command: Command) -> Result<impl warp::Reply, Infallible> {
    let response = match command {
        // User commands
        Command::CreateUser { name, email } => create_user(&pool, name, email).await,
        Command::GetUser { id } => get_user(&pool, id).await,
        Command::ListUsers => list_users(&pool).await,
        Command::UpdateUser { id, name } => update_user(&pool, id, name).await,
        Command::DeleteUser { id } => delete_user(&pool, id).await,

        // Role commands
        Command::CreateRole { slug, name } => create_role(&pool, slug, name).await,
        Command::GetRole { slug } => get_role(&pool, slug).await,
        Command::ListRoles => list_roles(&pool).await,
        Command::UpdateRole { slug, name } => update_role(&pool, slug, name).await,
        Command::DeleteRole { slug } => delete_role(&pool, slug).await,

        // User-Role relationship commands
        Command::AddRoleToUser { user_id, role_slug } => {
            add_role_to_user(&pool, user_id, role_slug).await
        }
        Command::RemoveRoleFromUser { user_id, role_slug } => {
            remove_role_from_user(&pool, user_id, role_slug).await
        }
        Command::GetUserRoles { user_id } => get_user_roles(&pool, user_id).await,
    };

    Ok(warp::reply::json(&response))
}

async fn create_user(pool: &DbPool, name: String, email: String) -> Response {
    match sqlx::query("INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username")
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await
    {
        Ok(row) => {
            let user = shared::User {
                id: row.get("id"),
                name: row.get("username"),
            };
            Response::Success {
                message: "User created successfully".to_string(),
                data: Some(serde_json::to_value(user).unwrap()),
            }
        }
        Err(e) => Response::Error {
            message: format!("Failed to create user: {}", e),
        },
    }
}

async fn get_user(pool: &DbPool, id: i32) -> Response {
    match sqlx::query("SELECT id, username FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(row)) => {
            let user = shared::User {
                id: row.get("id"),
                name: row.get("username"),
            };
            Response::Success {
                message: "User found".to_string(),
                data: Some(serde_json::to_value(user).unwrap()),
            }
        }
        Ok(None) => Response::Error {
            message: "User not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to get user: {}", e),
        },
    }
}

async fn list_users(pool: &DbPool) -> Response {
    match sqlx::query("SELECT id, username FROM users ORDER BY id")
        .fetch_all(pool)
        .await
    {
        Ok(rows) => {
            let users: Vec<shared::User> = rows
                .into_iter()
                .map(|row| shared::User {
                    id: row.get("id"),
                    name: row.get("username"),
                })
                .collect();
            Response::Success {
                message: format!("Found {} users", users.len()),
                data: Some(serde_json::to_value(users).unwrap()),
            }
        }
        Err(e) => Response::Error {
            message: format!("Failed to list users: {}", e),
        },
    }
}

async fn update_user(pool: &DbPool, id: i32, name: String) -> Response {
    match sqlx::query("UPDATE users SET username = $1 WHERE id = $2 RETURNING id, username")
        .bind(name)
        .bind(id)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(row)) => {
            let user = shared::User {
                id: row.get("id"),
                name: row.get("username"),
            };
            Response::Success {
                message: "User updated successfully".to_string(),
                data: Some(serde_json::to_value(user).unwrap()),
            }
        }
        Ok(None) => Response::Error {
            message: "User not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to update user: {}", e),
        },
    }
}

async fn delete_user(pool: &DbPool, id: i32) -> Response {
    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Response::Success {
            message: "User deleted successfully".to_string(),
            data: None,
        },
        Ok(_) => Response::Error {
            message: "User not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to delete user: {}", e),
        },
    }
}

// Role CRUD operations
async fn create_role(pool: &DbPool, slug: String, name: String) -> Response {
    match sqlx::query("INSERT INTO roles (slug, name) VALUES ($1, $2) RETURNING slug, name")
        .bind(slug)
        .bind(name)
        .fetch_one(pool)
        .await
    {
        Ok(row) => {
            let role = shared::Role {
                slug: row.get("slug"),
                name: row.get("name"),
            };
            Response::Success {
                message: "Role created successfully".to_string(),
                data: Some(serde_json::to_value(role).unwrap()),
            }
        }
        Err(e) => Response::Error {
            message: format!("Failed to create role: {}", e),
        },
    }
}

async fn get_role(pool: &DbPool, slug: String) -> Response {
    match sqlx::query("SELECT slug, name FROM roles WHERE slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(row)) => {
            let role = shared::Role {
                slug: row.get("slug"),
                name: row.get("name"),
            };
            Response::Success {
                message: "Role found".to_string(),
                data: Some(serde_json::to_value(role).unwrap()),
            }
        }
        Ok(None) => Response::Error {
            message: "Role not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to get role: {}", e),
        },
    }
}

async fn list_roles(pool: &DbPool) -> Response {
    match sqlx::query("SELECT slug, name FROM roles ORDER BY slug")
        .fetch_all(pool)
        .await
    {
        Ok(rows) => {
            let roles: Vec<shared::Role> = rows
                .into_iter()
                .map(|row| shared::Role {
                    slug: row.get("slug"),
                    name: row.get("name"),
                })
                .collect();
            Response::Success {
                message: format!("Found {} roles", roles.len()),
                data: Some(serde_json::to_value(roles).unwrap()),
            }
        }
        Err(e) => Response::Error {
            message: format!("Failed to list roles: {}", e),
        },
    }
}

async fn update_role(pool: &DbPool, slug: String, name: String) -> Response {
    match sqlx::query("UPDATE roles SET name = $1 WHERE slug = $2 RETURNING slug, name")
        .bind(name)
        .bind(slug)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(row)) => {
            let role = shared::Role {
                slug: row.get("slug"),
                name: row.get("name"),
            };
            Response::Success {
                message: "Role updated successfully".to_string(),
                data: Some(serde_json::to_value(role).unwrap()),
            }
        }
        Ok(None) => Response::Error {
            message: "Role not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to update role: {}", e),
        },
    }
}

async fn delete_role(pool: &DbPool, slug: String) -> Response {
    match sqlx::query("DELETE FROM roles WHERE slug = $1")
        .bind(slug)
        .execute(pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Response::Success {
            message: "Role deleted successfully".to_string(),
            data: None,
        },
        Ok(_) => Response::Error {
            message: "Role not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to delete role: {}", e),
        },
    }
}

// User-Role relationship operations
async fn add_role_to_user(pool: &DbPool, user_id: i32, role_slug: String) -> Response {
    match sqlx::query("INSERT INTO users_roles (user_id, role_slug) VALUES ($1, $2)")
        .bind(user_id)
        .bind(role_slug)
        .execute(pool)
        .await
    {
        Ok(_) => Response::Success {
            message: "Role added to user successfully".to_string(),
            data: None,
        },
        Err(e) => Response::Error {
            message: format!("Failed to add role to user: {}", e),
        },
    }
}

async fn remove_role_from_user(pool: &DbPool, user_id: i32, role_slug: String) -> Response {
    match sqlx::query("DELETE FROM users_roles WHERE user_id = $1 AND role_slug = $2")
        .bind(user_id)
        .bind(role_slug)
        .execute(pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Response::Success {
            message: "Role removed from user successfully".to_string(),
            data: None,
        },
        Ok(_) => Response::Error {
            message: "User-role relationship not found".to_string(),
        },
        Err(e) => Response::Error {
            message: format!("Failed to remove role from user: {}", e),
        },
    }
}

async fn get_user_roles(pool: &DbPool, user_id: i32) -> Response {
    match sqlx::query(
        "SELECT r.slug, r.name FROM roles r 
         INNER JOIN users_roles ur ON r.slug = ur.role_slug 
         WHERE ur.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    {
        Ok(rows) => {
            let roles: Vec<shared::Role> = rows
                .into_iter()
                .map(|row| shared::Role {
                    slug: row.get("slug"),
                    name: row.get("name"),
                })
                .collect();
            Response::Success {
                message: format!("Found {} roles for user", roles.len()),
                data: Some(serde_json::to_value(roles).unwrap()),
            }
        }
        Err(e) => Response::Error {
            message: format!("Failed to get user roles: {}", e),
        },
    }
}
