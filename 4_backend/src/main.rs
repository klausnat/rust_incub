mod models;
mod graphql;
mod repository;
mod schema;

use dotenvy::dotenv;
use std::env;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::GraphQLResponse;
use warp::{http::Response, Filter};
use schema::{AppSchema, Query, Mutation};

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Инициализация базы данных
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Создаем репозиторий
    let repository = repository::Repository::new(pool.clone());

    // Создаем GraphQL схему
    let schema = AppSchema::build(Query, Mutation, async_graphql::EmptySubscription)
        .data(repository)
        .finish();

    // Маршруты
    let graphql_post = async_graphql_warp::graphql(schema)
        .and_then(|(schema, request): (AppSchema, async_graphql::Request)| async move {
            Ok::<_, std::convert::Infallible>(GraphQLResponse::from(schema.execute(request).await))
        });

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .with(warp::cors().allow_any_origin());

    println!("GraphQL Playground: http://localhost:8000");
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;
}