use actix_web::{web, App, HttpServer, Result, HttpResponse};
use shared::{
    User, CreateUserRequest, UpdateUserRequest, Role, CreateRoleRequest, UpdateRoleRequest,
    AddRoleToUserRequest, ApiResponse,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

type DbPool = Pool<Postgres>;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_users,
        create_user,
        get_user,
        update_user,
        delete_user,
        get_roles,
        create_role,
        get_role,
        update_role,
        delete_role,
        get_user_roles,
        add_role_to_user,
        remove_role_from_user
    ),
    components(schemas(
        User,
        CreateUserRequest,
        UpdateUserRequest,
        Role,
        CreateRoleRequest,
        UpdateRoleRequest,
        AddRoleToUserRequest,
        ApiResponse<User>,
        ApiResponse<Vec<User>>,
        ApiResponse<Role>,
        ApiResponse<Vec<Role>>,
        ApiResponse<String>
    ))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Database connection
    let database_url = "postgres://rust_user:rust_password@localhost/rust_cli_app";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database successfully!");
    println!("Server starting on http://localhost:8080");
    println!("API Documentation: http://localhost:8080/swagger-ui/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(get_users)
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_roles)
            .service(create_role)
            .service(get_role)
            .service(update_role)
            .service(delete_role)
            .service(get_user_roles)
            .service(add_role_to_user)
            .service(remove_role_from_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// User endpoints
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of all users", body = ApiResponse<Vec<User>>)
    )
)]
#[actix_web::get("/api/users")]
async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match sqlx::query_as::<_, User>("SELECT id, username, email, created_at FROM users ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => Ok(HttpResponse::Ok().json(ApiResponse::success(users, "Users retrieved successfully"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<User>>::error(&format!("Failed to get users: {}", e)))),
    }
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = ApiResponse<User>),
        (status = 400, description = "Bad request")
    )
)]
#[actix_web::post("/api/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email, created_at"
    )
    .bind(&user_data.username)
    .bind(&user_data.email)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(user) => Ok(HttpResponse::Created().json(ApiResponse::success(user, "User created successfully"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<User>::error(&format!("Failed to create user: {}", e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User found", body = ApiResponse<User>),
        (status = 404, description = "User not found")
    )
)]
#[actix_web::get("/api/users/{id}")]
async fn get_user(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse> {
    match sqlx::query_as::<_, User>("SELECT id, username, email, created_at FROM users WHERE id = $1")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(ApiResponse::success(user, "User found"))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<User>::error("User not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<User>::error(&format!("Failed to get user: {}", e)))),
    }
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = ApiResponse<User>),
        (status = 404, description = "User not found")
    )
)]
#[actix_web::put("/api/users/{id}")]
async fn update_user(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    user_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    match sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1 WHERE id = $2 RETURNING id, username, email, created_at"
    )
    .bind(&user_data.username)
    .bind(*id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(ApiResponse::success(user, "User updated successfully"))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<User>::error("User not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<User>::error(&format!("Failed to update user: {}", e)))),
    }
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User deleted successfully", body = ApiResponse<String>),
        (status = 404, description = "User not found")
    )
)]
#[actix_web::delete("/api/users/{id}")]
async fn delete_user(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse> {
    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            Ok(HttpResponse::Ok().json(ApiResponse::success("".to_string(), "User deleted successfully")))
        }
        Ok(_) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("User not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&format!("Failed to delete user: {}", e)))),
    }
}

// Role endpoints
#[utoipa::path(
    get,
    path = "/api/roles",
    responses(
        (status = 200, description = "List of all roles", body = ApiResponse<Vec<Role>>)
    )
)]
#[actix_web::get("/api/roles")]
async fn get_roles(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match sqlx::query_as::<_, Role>("SELECT slug, name FROM roles ORDER BY slug")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(roles) => Ok(HttpResponse::Ok().json(ApiResponse::success(roles, "Roles retrieved successfully"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<Role>>::error(&format!("Failed to get roles: {}", e)))),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created successfully", body = ApiResponse<Role>),
        (status = 400, description = "Bad request")
    )
)]
#[actix_web::post("/api/roles")]
async fn create_role(
    pool: web::Data<DbPool>,
    role_data: web::Json<CreateRoleRequest>,
) -> Result<HttpResponse> {
    match sqlx::query_as::<_, Role>(
        "INSERT INTO roles (slug, name) VALUES ($1, $2) RETURNING slug, name"
    )
    .bind(&role_data.slug)
    .bind(&role_data.name)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(role) => Ok(HttpResponse::Created().json(ApiResponse::success(role, "Role created successfully"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<Role>::error(&format!("Failed to create role: {}", e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/{slug}",
    responses(
        (status = 200, description = "Role found", body = ApiResponse<Role>),
        (status = 404, description = "Role not found")
    )
)]
#[actix_web::get("/api/roles/{slug}")]
async fn get_role(pool: web::Data<DbPool>, slug: web::Path<String>) -> Result<HttpResponse> {
    match sqlx::query_as::<_, Role>("SELECT slug, name FROM roles WHERE slug = $1")
        .bind(slug.into_inner())
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(role)) => Ok(HttpResponse::Ok().json(ApiResponse::success(role, "Role found"))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<Role>::error("Role not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Role>::error(&format!("Failed to get role: {}", e)))),
    }
}

#[utoipa::path(
    put,
    path = "/api/roles/{slug}",
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated successfully", body = ApiResponse<Role>),
        (status = 404, description = "Role not found")
    )
)]
#[actix_web::put("/api/roles/{slug}")]
async fn update_role(
    pool: web::Data<DbPool>,
    slug: web::Path<String>,
    role_data: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse> {
    match sqlx::query_as::<_, Role>(
        "UPDATE roles SET name = $1 WHERE slug = $2 RETURNING slug, name"
    )
    .bind(&role_data.name)
    .bind(slug.into_inner())
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(role)) => Ok(HttpResponse::Ok().json(ApiResponse::success(role, "Role updated successfully"))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<Role>::error("Role not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Role>::error(&format!("Failed to update role: {}", e)))),
    }
}

#[utoipa::path(
    delete,
    path = "/api/roles/{slug}",
    responses(
        (status = 200, description = "Role deleted successfully", body = ApiResponse<String>),
        (status = 404, description = "Role not found")
    )
)]
#[actix_web::delete("/api/roles/{slug}")]
async fn delete_role(pool: web::Data<DbPool>, slug: web::Path<String>) -> Result<HttpResponse> {
    match sqlx::query("DELETE FROM roles WHERE slug = $1")
        .bind(slug.into_inner())
        .execute(pool.get_ref())
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            Ok(HttpResponse::Ok().json(ApiResponse::success("".to_string(), "Role deleted successfully")))
        }
        Ok(_) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Role not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&format!("Failed to delete role: {}", e)))),
    }
}

// User-Role relationship endpoints
#[utoipa::path(
    get,
    path = "/api/users/{user_id}/roles",
    responses(
        (status = 200, description = "User roles retrieved", body = ApiResponse<Vec<Role>>),
        (status = 404, description = "User not found")
    )
)]
#[actix_web::get("/api/users/{user_id}/roles")]
async fn get_user_roles(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> Result<HttpResponse> {
    match sqlx::query_as::<_, Role>(
        "SELECT r.slug, r.name FROM roles r 
         INNER JOIN users_roles ur ON r.slug = ur.role_slug 
         WHERE ur.user_id = $1"
    )
    .bind(*user_id)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(roles) => Ok(HttpResponse::Ok().json(ApiResponse::success(roles, "User roles retrieved successfully"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<Role>>::error(&format!("Failed to get user roles: {}", e)))),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/{user_id}/roles",
    request_body = AddRoleToUserRequest,
    responses(
        (status = 201, description = "Role added to user successfully", body = ApiResponse<String>),
        (status = 400, description = "Bad request")
    )
)]
#[actix_web::post("/api/users/{user_id}/roles")]
async fn add_role_to_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    request: web::Json<AddRoleToUserRequest>,
) -> Result<HttpResponse> {
    match sqlx::query("INSERT INTO users_roles (user_id, role_slug) VALUES ($1, $2)")
        .bind(*user_id)
        .bind(&request.role_slug)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success("".to_string(), "Role added to user successfully"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<String>::error(&format!("Failed to add role to user: {}", e)))),
    }
}

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}/roles/{role_slug}",
    responses(
        (status = 200, description = "Role removed from user successfully", body = ApiResponse<String>),
        (status = 404, description = "Relationship not found")
    )
)]
#[actix_web::delete("/api/users/{user_id}/roles/{role_slug}")]
async fn remove_role_from_user(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, String)>,
) -> Result<HttpResponse> {
    let (user_id, role_slug) = path.into_inner();
    
    match sqlx::query("DELETE FROM users_roles WHERE user_id = $1 AND role_slug = $2")
        .bind(user_id)
        .bind(role_slug)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            Ok(HttpResponse::Ok().json(ApiResponse::success("".to_string(), "Role removed from user successfully")))
        }
        Ok(_) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("User-role relationship not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&format!("Failed to remove role from user: {}", e)))),
    }
}