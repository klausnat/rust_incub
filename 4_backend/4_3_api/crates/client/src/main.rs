use clap::{Parser, Subcommand};
use shared::{
    User, CreateUserRequest, UpdateUserRequest, Role, CreateRoleRequest, UpdateRoleRequest,
    AddRoleToUserRequest, ApiResponse,
};
use reqwest::Client;
use std::process;

const BASE_URL: &str = "http://localhost:8080/api";

#[derive(Parser)]
#[command(name = "user-management-cli")]
#[command(about = "CLI client for user management API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// User management commands
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
    /// Role management commands  
    Role {
        #[command(subcommand)]
        action: RoleCommands,
    },
    /// User-Role relationship commands
    UserRole {
        #[command(subcommand)]
        action: UserRoleCommands,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// Create a new user
    Create { username: String, email: String },
    /// Get a user by ID
    Get { id: i32 },
    /// List all users
    List,
    /// Update a user
    Update { id: i32, username: String },
    /// Delete a user
    Delete { id: i32 },
}

#[derive(Subcommand)]
enum RoleCommands {
    /// Create a new role
    Create { slug: String, name: String },
    /// Get a role by slug
    Get { slug: String },
    /// List all roles
    List,
    /// Update a role
    Update { slug: String, name: String },
    /// Delete a role
    Delete { slug: String },
}

#[derive(Subcommand)]
enum UserRoleCommands {
    /// Add a role to a user
    Add { user_id: i32, role_slug: String },
    /// Remove a role from a user
    Remove { user_id: i32, role_slug: String },
    /// Get all roles for a user
    Get { user_id: i32 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    let http_client = Client::new();
    
    match cli.command {
        Commands::User { action } => handle_user_commands(http_client, action).await,
        Commands::Role { action } => handle_role_commands(http_client, action).await,
        Commands::UserRole { action } => handle_user_role_commands(http_client, action).await,
    }
}

async fn handle_user_commands(client: Client, action: UserCommands) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        UserCommands::Create { username, email } => {
            let request = CreateUserRequest { username, email };
            let response: ApiResponse<User> = client
                .post(&format!("{}/users", BASE_URL))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        UserCommands::Get { id } => {
            let response: ApiResponse<User> = client
                .get(&format!("{}/users/{}", BASE_URL, id))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        UserCommands::List => {
            let response: ApiResponse<Vec<User>> = client
                .get(&format!("{}/users", BASE_URL))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        UserCommands::Update { id, username } => {
            let request = UpdateUserRequest { username };
            let response: ApiResponse<User> = client
                .put(&format!("{}/users/{}", BASE_URL, id))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        UserCommands::Delete { id } => {
            let response: ApiResponse<String> = client
                .delete(&format!("{}/users/{}", BASE_URL, id))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
    }
    
    Ok(())
}

async fn handle_role_commands(client: Client, action: RoleCommands) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        RoleCommands::Create { slug, name } => {
            let request = CreateRoleRequest { slug, name };
            let response: ApiResponse<Role> = client
                .post(&format!("{}/roles", BASE_URL))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        RoleCommands::Get { slug } => {
            let response: ApiResponse<Role> = client
                .get(&format!("{}/roles/{}", BASE_URL, slug))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        RoleCommands::List => {
            let response: ApiResponse<Vec<Role>> = client
                .get(&format!("{}/roles", BASE_URL))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        RoleCommands::Update { slug, name } => {
            let request = UpdateRoleRequest { name };
            let response: ApiResponse<Role> = client
                .put(&format!("{}/roles/{}", BASE_URL, slug))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        RoleCommands::Delete { slug } => {
            let response: ApiResponse<String> = client
                .delete(&format!("{}/roles/{}", BASE_URL, slug))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
    }
    
    Ok(())
}

async fn handle_user_role_commands(client: Client, action: UserRoleCommands) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        UserRoleCommands::Add { user_id, role_slug } => {
            let request = AddRoleToUserRequest { user_id, role_slug };
            let response: ApiResponse<String> = client
                .post(&format!("{}/users/{}/roles", BASE_URL, user_id))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        UserRoleCommands::Remove { user_id, role_slug } => {
            let response: ApiResponse<String> = client
                .delete(&format!("{}/users/{}/roles/{}", BASE_URL, user_id, role_slug))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
        UserRoleCommands::Get { user_id } => {
            let response: ApiResponse<Vec<Role>> = client
                .get(&format!("{}/users/{}/roles", BASE_URL, user_id))
                .send()
                .await?
                .json()
                .await?;
            
            display_response(response);
        }
    }
    
    Ok(())
}

fn display_response<T: serde::Serialize>(response: ApiResponse<T>) {
    if response.success {
        println!("✅ {}", response.message);
        if let Some(data) = response.data {
            println!("{}", serde_json::to_string_pretty(&data).unwrap());
        }
    } else {
        eprintln!("❌ {}", response.message);
        process::exit(1);
    }
}