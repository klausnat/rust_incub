// use serde::{Deserialize, Serialize};

// // API request types
// #[derive(Debug, Serialize, Deserialize)]
// pub enum Command {
//     // User commands
//     CreateUser { name: String },
//     GetUser { id: i32 },
//     ListUsers,
//     UpdateUser { id: i32, name: String },
//     DeleteUser { id: i32 },
    
//     // Role commands  
//     CreateRole { slug: String, name: String },
//     GetRole { slug: String },
//     ListRoles,
//     UpdateRole { slug: String, name: String },
//     DeleteRole { slug: String },
    
//     // User-Role relationship commands
//     AddRoleToUser { user_id: i32, role_slug: String },
//     RemoveRoleFromUser { user_id: i32, role_slug: String },
//     GetUserRoles { user_id: i32 },
// }

// // API response type
// #[derive(Debug, Serialize, Deserialize)]
// pub enum Response {
//     Success { message: String, data: Option<serde_json::Value> },
//     Error { message: String },
// }

// // Data models
// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// pub struct User {
//     pub id: i32,
//     pub name: String,
// }

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// pub struct Role {
//     pub slug: String,
//     pub name: String,
// }

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// pub struct UserRole {
//     pub user_id: i32,
//     pub role_slug: String,
// }