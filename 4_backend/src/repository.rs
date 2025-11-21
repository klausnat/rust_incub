use sqlx::postgres::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{User, Friendship, UserWithFriends, AuthUser, NewUser};

#[derive(Clone)]
pub struct Repository {
    pub pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // User operations
    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, name, password_hash, created_at, updated_at FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, name, password_hash, created_at, updated_at FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, name, password_hash, created_at, updated_at FROM users WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, sqlx::Error> {
        let password_hash = hash(new_user.password, DEFAULT_COST)
            .map_err(|e| sqlx::Error::Protocol(format!("Password hashing failed: {}", e).into()))?;

        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, password_hash) VALUES ($1, $2) RETURNING id, name, password_hash, created_at, updated_at"
        )
        .bind(new_user.name)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn authenticate_user(&self, name: &str, password: &str) -> Result<Option<AuthUser>, sqlx::Error> {
        let user = self.get_user_by_name(name).await?;
        
        match user {
            Some(user) => {
                if verify(password, &user.password_hash).unwrap_or(false) {
                    Ok(Some(AuthUser {
                        id: user.id,
                        name: user.name,
                    }))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    // Friendship operations
    pub async fn get_all_friendships(&self) -> Result<Vec<Friendship>, sqlx::Error> {
        sqlx::query_as::<_, Friendship>("SELECT id, user_id, friend_id, created_at FROM friendships")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_friendships_by_user_id(&self, user_id: Uuid) -> Result<Vec<Friendship>, sqlx::Error> {
        sqlx::query_as::<_, Friendship>(
            "SELECT id, user_id, friend_id, created_at FROM friendships WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_friends_of_user(&self, user_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT u.id, u.name, u.password_hash, u.created_at, u.updated_at 
            FROM users u
            INNER JOIN friendships f ON u.id = f.friend_id
            WHERE f.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_friendship(&self, user_id: Uuid, friend_id: Uuid) -> Result<Friendship, sqlx::Error> {
        // Check if friendship already exists
        let existing_friendship = sqlx::query_as::<_, Friendship>(
            "SELECT id, user_id, friend_id, created_at FROM friendships WHERE user_id = $1 AND friend_id = $2"
        )
        .bind(user_id)
        .bind(friend_id)
        .fetch_optional(&self.pool)
        .await?;

        if existing_friendship.is_some() {
            return Err(sqlx::Error::Protocol("Friendship already exists".into()));
        }

        // Check if trying to befriend self
        if user_id == friend_id {
            return Err(sqlx::Error::Protocol("Cannot befriend yourself".into()));
        }

        sqlx::query_as::<_, Friendship>(
            "INSERT INTO friendships (user_id, friend_id) VALUES ($1, $2) RETURNING id, user_id, friend_id, created_at"
        )
        .bind(user_id)
        .bind(friend_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn remove_friendship(&self, friendship_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM friendships WHERE id = $1")
            .bind(friendship_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_friendship_by_users(&self, user_id: Uuid, friend_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM friendships WHERE user_id = $1 AND friend_id = $2")
            .bind(user_id)
            .bind(friend_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_user_with_friends(&self, user_id: Uuid) -> Result<UserWithFriends, sqlx::Error> {
        let user = self.get_user_by_id(user_id).await?.ok_or_else(|| {
            sqlx::Error::RowNotFound
        })?;
        
        let friends = self.get_friends_of_user(user_id).await?;

        Ok(UserWithFriends {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
            friends: Some(friends),
        })
    }

    pub async fn get_friendship_by_id(&self, friendship_id: Uuid) -> Result<Option<Friendship>, sqlx::Error> {
        sqlx::query_as::<_, Friendship>(
            "SELECT id, user_id, friend_id, created_at FROM friendships WHERE id = $1"
        )
        .bind(friendship_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn search_users_by_name(&self, name_pattern: &str) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, password_hash, created_at, updated_at FROM users WHERE name ILIKE $1"
        )
        .bind(format!("%{}%", name_pattern))
        .fetch_all(&self.pool)
        .await
    }

    // Check if two users are friends
    pub async fn are_friends(&self, user_id: Uuid, friend_id: Uuid) -> Result<bool, sqlx::Error> {
        let friendship = sqlx::query_as::<_, Friendship>(
            "SELECT id, user_id, friend_id, created_at FROM friendships WHERE user_id = $1 AND friend_id = $2"
        )
        .bind(user_id)
        .bind(friend_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(friendship.is_some())
    }
}

