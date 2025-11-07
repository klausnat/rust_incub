extern crate diesel;

mod db;
mod models;
mod schema;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use prettytable::{row, Table};

use crate::models::{
    NewRole, NewUser, NewUserRole, Role, UpdateRole, UpdateUser, User, UserRole,
};

#[derive(Parser)]
#[command(name = "task-cli")]
#[command(about = "A simple CLI application which allows to CRUD data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new user
    CreateUser {
        username: String,
        #[arg(short, long)]
        email: String,
        role_slugs: Vec<String>,
    },
    /// List all users
    ListUsers {},
    /// Update user
    UpdateUser {
        id: i32,
        #[arg(short, long)]
        username: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Delete a user
    DeleteUser {
        id: i32,
    },

    /// Create a new role
    CreateRole {
        name: String,
        #[arg(short, long)]
        permissions: Option<String>,
    },
    /// List all roles
    ListRoles {},
    /// Update role
    UpdateRole {
        slug: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        permissions: Option<String>,
    },
    /// Delete a role
    DeleteRole {
        slug: String,
    },

    AssignRoleToUser {
        user_id: i32,
        role_slug: String,
    },

    ShowUserWithRoles {
        user_id: i32,
    },

    SelectRolesBySlug {
        role_slug: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut connection = db::establish_connection();

    match cli.command {
        Commands::CreateUser {
            username,
            email,
            role_slugs,
        } => {
            create_user(&mut connection, username, email, role_slugs)?;
        }
        Commands::ListUsers {} => {
            list_users(&mut connection)?;
        }
        Commands::UpdateUser {
            id,
            username,
            email,
        } => {
            update_user(&mut connection, id, username, email)?;
        }
        Commands::DeleteUser { id } => {
            delete_user(&mut connection, id)?;
        }
        Commands::CreateRole { name, permissions } => {
            create_role(&mut connection, name, permissions)?;
        }
        Commands::ListRoles {} => {
            list_roles(&mut connection)?;
        }
        Commands::UpdateRole {
            slug,
            name,
            permissions,
        } => {
            update_role(&mut connection, slug, name, permissions)?;
        }
        Commands::DeleteRole { slug } => {
            delete_role(&mut connection, slug)?;
        }
        Commands::AssignRoleToUser { user_id, role_slug } => {
            assign_role_to_user(&mut connection, user_id, &role_slug)?;
        }
        Commands::ShowUserWithRoles { user_id } => {
            show_user_with_roles(&mut connection, user_id)?;
        }
        Commands::SelectRolesBySlug { role_slug } => {
            show_roles_by_slug(&mut connection, &role_slug)?;
        }
    }

    Ok(())
}

fn create_user(
    conn: &mut PgConnection,
    username: String,
    email: String,
    role_slugs: Vec<String>,
) -> Result<()> {
    use crate::schema::{roles, users, users_roles};

    conn.transaction(|conn| {
        // Create user
        let new_user = NewUser { username, email };
        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)?;

        // Assign roles
        for role_slug in role_slugs {
            // Verify role exists
            let role = roles::table
                .find(role_slug.clone())
                .first::<Role>(conn)
                .map_err(|_| anyhow::anyhow!("Role '{}' not found", role_slug))?; // Change here

            // Assign role to user
            let user_role = NewUserRole {
                user_id: user.id,
                role_slug: role.slug.clone(),
            };

            diesel::insert_into(users_roles::table)
                .values(&user_role)
                .execute(conn)?;
        }

        Ok(())
    })
}

fn list_users(conn: &mut PgConnection) -> Result<()> {
    use crate::schema::users::dsl::*;

    let query = users.into_boxed();

    let users_list = query.load::<User>(conn)?;

    let mut table = Table::new();
    table.add_row(row!["ID", "Username", "Email", "Created At"]);

    for user in users_list {
        table.add_row(row![
            user.id,
            user.username,
            user.email,
            user.created_at.format("%Y-%m-%d %H:%M")
        ]);
    }

    table.printstd();
    Ok(())
}

fn update_user(
    conn: &mut PgConnection,
    user_id: i32,
    new_username: Option<String>,
    new_email: Option<String>,
) -> Result<()> {
    use crate::schema::users::dsl::*;

    let update_user = UpdateUser {
        username: new_username,
        email: new_email,
    };

    let affected_rows = diesel::update(users.find(user_id))
        .set(&update_user)
        .execute(conn)?;

    if affected_rows > 0 {
        println!("User {} updated successfully!", user_id);
    } else {
        println!("User {} not found!", user_id);
    }

    Ok(())
}

fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<()> {
    use crate::schema::users::dsl::*;

    let affected_rows = diesel::delete(users.find(user_id)).execute(conn)?;

    if affected_rows > 0 {
        println!("User {} deleted successfully!", user_id);
    } else {
        println!("User {} not found!", user_id);
    }

    Ok(())
}

/// manipulations with roles

fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' => '-',
            _ => '\0', // Replace other characters with null (to be filtered out)
        })
        .filter(|c| *c != '\0') // Remove null characters
        .collect::<String>()
        .trim_matches('-') // Remove leading/trailing hyphens
        .to_string()
}

fn create_role(conn: &mut PgConnection, name: String, permissions: Option<String>) -> Result<()> {
    use crate::schema::roles;

    let slug = generate_slug(&name);

    let new_role = NewRole {
        slug,
        name,
        permissions,
    };

    diesel::insert_into(roles::table)
        .values(&new_role)
        .execute(conn)?;

    println!("Role created successfully!");
    Ok(())
}

fn list_roles(conn: &mut PgConnection) -> Result<()> {
    use crate::schema::roles::dsl::*;

    let query = roles.into_boxed();

    let roles_list = query.load::<Role>(conn)?;

    let mut table = Table::new();
    table.add_row(row!["Slug", "Name", "Permissions"]);

    for role in roles_list {
        table.add_row(row![
            role.slug,
            role.name,
            role.permissions.unwrap_or(String::from("NULL")),
        ]);
    }

    table.printstd();
    Ok(())
}

fn update_role(
    conn: &mut PgConnection,
    role_slug: String,
    new_name: Option<String>,
    new_permissions: Option<String>,
) -> Result<()> {
    use crate::schema::roles::dsl::*;

    let update_role = UpdateRole {
        name: new_name,
        permissions: new_permissions,
    };

    let affected_rows = diesel::update(roles.find(role_slug.clone()))
        .set(&update_role)
        .execute(conn)?;

    if affected_rows > 0 {
        println!("Role {} updated successfully!", role_slug);
    } else {
        println!("Role {} not found!", role_slug);
    }

    Ok(())
}

fn delete_role(conn: &mut PgConnection, role_slug: String) -> Result<()> {
    use crate::schema::roles::dsl::*;

    let affected_rows = diesel::delete(roles.filter(slug.eq(role_slug.clone()))).execute(conn)?;

    if affected_rows > 0 {
        println!("Role {} deleted successfully!", role_slug);
    } else {
        println!("Role {} not found!", role_slug);
    }

    Ok(())
}

pub fn assign_role_to_user(
    conn: &mut PgConnection,
    user_id: i32,
    role_slug: &str,
) -> Result<UserRole, diesel::result::Error> {
    use crate::schema::users_roles;

    // Verify user exists
    crate::schema::users::table
        .find(user_id)
        .first::<User>(conn)?;

    // Verify role exists
    crate::schema::roles::table
        .find(role_slug)
        .first::<Role>(conn)?;

    // Assign role
    let new_assignment = NewUserRole {
        user_id,
        role_slug: role_slug.to_string(),
    };

    diesel::insert_into(users_roles::table)
        .values(&new_assignment)
        .get_result(conn)
}

pub fn show_user_with_roles(conn: &mut PgConnection, req_user_id: i32) -> Result<()> {
    use crate::schema::{roles::dsl::*, users::dsl::*, users_roles::dsl::*};

    // Get user
    let user = users
        .find(req_user_id)
        .first::<User>(conn)
        .map_err(|_| diesel::result::Error::NotFound)?;

    // Get user's roles
    let user_roles = users_roles
        .filter(user_id.eq(req_user_id))
        .inner_join(roles.on(role_slug.eq(slug)))
        .select((slug, name, permissions))
        .load::<Role>(conn)?;

    // Create and display table
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
    table.set_titles(row![
        "User ID",
        "Username",
        "Email",
        "Roles Count",
        "Role Slugs"
    ]);

    let role_slugs: Vec<String> = user_roles.iter().map(|r| r.slug.clone()).collect();
    let roles_display = if role_slugs.is_empty() {
        "No roles".to_string()
    } else {
        role_slugs.join(", ")
    };

    table.add_row(row![
        user.id,
        user.username,
        user.email,
        user_roles.len(),
        roles_display
    ]);

    table.printstd();
    Ok(())
}

pub fn show_roles_by_slug(conn: &mut PgConnection, role_slug_pattern: &str) -> Result<()> {
    use crate::schema::roles::dsl::*; // Import table columns

    let search_pattern = format!("%{}%", role_slug_pattern);

    // find all roles matching
    let query = roles.filter(slug.like(search_pattern));

    let roles_list = query.load::<Role>(conn)?;

    let mut table = Table::new();
    table.add_row(row!["Slug", "Name", "Permissions"]);

    for role in roles_list {
        table.add_row(row![
            role.slug,
            role.name,
            role.permissions.unwrap_or(String::from("NULL")),
        ]);
    }

    table.printstd();
    Ok(())
}
