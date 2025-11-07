use clap::{Parser, Subcommand};
use shared::{Command, Response};
use reqwest::Client;
use std::process;

#[derive(Parser)]
#[command(name = "database-cli")]
#[command(about = "CLI client for database operations", long_about = None)]
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
    Create { name: String, email: String },
    Get { id: i32 },
    List,
    Update { id: i32, name: String },
    Delete { id: i32 },
}

#[derive(Subcommand)]
enum RoleCommands {
    Create { slug: String, name: String },
    Get { slug: String },
    List,
    Update { slug: String, name: String },
    Delete { slug: String },
}

#[derive(Subcommand)]
enum UserRoleCommands {
    Add { user_id: i32, role_slug: String },
    Remove { user_id: i32, role_slug: String },
    Get { user_id: i32 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Create HTTP client
    let http_client = Client::new();
    let server_url = "http://localhost:3030/api/command";
    
    // Convert CLI commands to API commands
    let command = match cli.command {
        Commands::User { action } => match action {
            UserCommands::Create { name, email } => Command::CreateUser { name, email },
            UserCommands::Get { id } => Command::GetUser { id },
            UserCommands::List => Command::ListUsers,
            UserCommands::Update { id, name } => Command::UpdateUser { id, name },
            UserCommands::Delete { id } => Command::DeleteUser { id },
        },
        Commands::Role { action } => match action {
            RoleCommands::Create { slug, name } => Command::CreateRole { slug, name },
            RoleCommands::Get { slug } => Command::GetRole { slug },
            RoleCommands::List => Command::ListRoles,
            RoleCommands::Update { slug, name } => Command::UpdateRole { slug, name },
            RoleCommands::Delete { slug } => Command::DeleteRole { slug },
        },
        Commands::UserRole { action } => match action {
            UserRoleCommands::Add { user_id, role_slug } => Command::AddRoleToUser { user_id, role_slug },
            UserRoleCommands::Remove { user_id, role_slug } => Command::RemoveRoleFromUser { user_id, role_slug },
            UserRoleCommands::Get { user_id } => Command::GetUserRoles { user_id },
        },
    };
    
    // Send command to server and display response
    send_command(&http_client, server_url, command).await?;
    
    Ok(())
}

// Thin client: just sends command and displays response
async fn send_command(
    client: &Client,
    url: &str,
    command: Command,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .post(url)
        .json(&command)
        .send()
        .await?;
    
    if response.status().is_success() {
        let api_response: Response = response.json().await?;
        display_response(api_response);
    } else {
        eprintln!("Server error: {}", response.status());
        process::exit(1);
    }
    
    Ok(())
}

// Display server response in a user-friendly format
fn display_response(response: Response) {
    match response {
        Response::Success { message, data } => {
            println!("✅ {}", message);
            if let Some(data) = data {
                println!("{}", serde_json::to_string_pretty(&data).unwrap());
            }
        }
        Response::Error { message } => {
            eprintln!("❌ {}", message);
            process::exit(1);
        }
    }
}