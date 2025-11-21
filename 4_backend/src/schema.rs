use async_graphql::*;
use uuid::Uuid;

use crate::repository::Repository;
use crate::graphql::{
    UserObject, FriendshipObject, UserWithFriendsObject, FriendshipWithDetailsObject,
    CreateUserInput, LoginInput, AddFriendInput, RemoveFriendInput, UpdateUserInput,
    AuthPayload, FriendshipOperationResult, UserOperationResult
};
use crate::models::NewUser;

pub struct Query;

#[Object]
impl Query {
    // Get all users
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>> {
        let repo = ctx.data::<Repository>()?;
        let users = repo.get_all_users().await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    // Get user by ID
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<UserObject>> {
        let repo = ctx.data::<Repository>()?;
        let uuid = Uuid::parse_str(&id).map_err(|e| Error::new(e.to_string()))?;
        
        let user = repo.get_user_by_id(uuid).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(user.map(|u| u.into()))
    }

    // Get user by name
    async fn user_by_name(&self, ctx: &Context<'_>, name: String) -> Result<Option<UserObject>> {
        let repo = ctx.data::<Repository>()?;
        let user = repo.get_user_by_name(&name).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(user.map(|u| u.into()))
    }

    // Get user with friends
    async fn user_with_friends(&self, ctx: &Context<'_>, id: ID) -> Result<Option<UserWithFriendsObject>> {
        let repo = ctx.data::<Repository>()?;
        let uuid = Uuid::parse_str(&id).map_err(|e| Error::new(e.to_string()))?;
        
        let user_with_friends = repo.get_user_with_friends(uuid).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(Some(user_with_friends.into()))
    }

    // Get all friendships
    async fn friendships(&self, ctx: &Context<'_>) -> Result<Vec<FriendshipObject>> {
        let repo = ctx.data::<Repository>()?;
        let friendships = repo.get_all_friendships().await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(friendships.into_iter().map(|f| f.into()).collect())
    }

    // Get friendships for a user
    async fn friendships_by_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<FriendshipObject>> {
        let repo = ctx.data::<Repository>()?;
        let uuid = Uuid::parse_str(&user_id).map_err(|e| Error::new(e.to_string()))?;
        
        let friendships = repo.get_friendships_by_user_id(uuid).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(friendships.into_iter().map(|f| f.into()).collect())
    }

    // Get friends of a user
    async fn friends(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<UserObject>> {
        let repo = ctx.data::<Repository>()?;
        let uuid = Uuid::parse_str(&user_id).map_err(|e| Error::new(e.to_string()))?;
        
        let friends = repo.get_friends_of_user(uuid).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(friends.into_iter().map(|f| f.into()).collect())
    }

    // Search users by name pattern
    async fn search_users(&self, ctx: &Context<'_>, name_pattern: String) -> Result<Vec<UserObject>> {
        let repo = ctx.data::<Repository>()?;
        let users = repo.search_users_by_name(&name_pattern).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    // Check if two users are friends
    async fn are_friends(&self, ctx: &Context<'_>, user_id: ID, friend_id: ID) -> Result<bool> {
        let repo = ctx.data::<Repository>()?;
        let user_uuid = Uuid::parse_str(&user_id).map_err(|e| Error::new(e.to_string()))?;
        let friend_uuid = Uuid::parse_str(&friend_id).map_err(|e| Error::new(e.to_string()))?;
        
        let are_friends = repo.are_friends(user_uuid, friend_uuid).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(are_friends)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // Create a new user
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<UserObject> {
        let repo = ctx.data::<Repository>()?;
        
        let new_user = NewUser {
            name: input.name,
            password: input.password,
        };
        
        let user = repo.create_user(new_user).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(user.into())
    }

    // Login user
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let repo = ctx.data::<Repository>()?;
        
        let auth_user = repo.authenticate_user(&input.name, &input.password).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        match auth_user {
            Some(user) => {
                // @TODO generate a JWT token here. Research which crate is the best
                let token = format!("mock_jwt_token_for_user_{}", user.id);
                
                Ok(AuthPayload {
                    token,
                    user: user.into(),
                })
            }
            None => Err(Error::new("Invalid credentials")),
        }
    }

    // Add a friend
    async fn add_friend(&self, ctx: &Context<'_>, user_id: ID, input: AddFriendInput) -> Result<FriendshipOperationResult> {
        let repo = ctx.data::<Repository>()?;
        
        let user_uuid = Uuid::parse_str(&user_id).map_err(|e| Error::new(e.to_string()))?;
        
        // Find the friend by name
        let friend = repo.get_user_by_name(&input.friend_name).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        match friend {
            Some(friend_user) => {
                match repo.create_friendship(user_uuid, friend_user.id).await {
                    Ok(friendship) => Ok(FriendshipOperationResult {
                        success: true,
                        message: "Friend added successfully".to_string(),
                        friendship: Some(friendship.into()),
                    }),
                    Err(e) => Ok(FriendshipOperationResult {
                        success: false,
                        message: e.to_string(),
                        friendship: None,
                    }),
                }
            }
            None => Ok(FriendshipOperationResult {
                success: false,
                message: "User not found".to_string(),
                friendship: None,
            }),
        }
    }

    // Remove a friend by friendship ID
    async fn remove_friend(&self, ctx: &Context<'_>, input: RemoveFriendInput) -> Result<FriendshipOperationResult> {
        let repo = ctx.data::<Repository>()?;
        
        let friendship_id = Uuid::parse_str(&input.friendship_id).map_err(|e| Error::new(e.to_string()))?;
        
        match repo.remove_friendship(friendship_id).await {
            Ok(true) => Ok(FriendshipOperationResult {
                success: true,
                message: "Friend removed successfully".to_string(),
                friendship: None,
            }),
            Ok(false) => Ok(FriendshipOperationResult {
                success: false,
                message: "Friendship not found".to_string(),
                friendship: None,
            }),
            Err(e) => Ok(FriendshipOperationResult {
                success: false,
                message: e.to_string(),
                friendship: None,
            }),
        }
    }

    // Remove a friend by user IDs
    async fn remove_friend_by_users(&self, ctx: &Context<'_>, user_id: ID, friend_id: ID) -> Result<FriendshipOperationResult> {
        let repo = ctx.data::<Repository>()?;
        
        let user_uuid = Uuid::parse_str(&user_id).map_err(|e| Error::new(e.to_string()))?;
        let friend_uuid = Uuid::parse_str(&friend_id).map_err(|e| Error::new(e.to_string()))?;
        
        match repo.remove_friendship_by_users(user_uuid, friend_uuid).await {
            Ok(true) => Ok(FriendshipOperationResult {
                success: true,
                message: "Friend removed successfully".to_string(),
                friendship: None,
            }),
            Ok(false) => Ok(FriendshipOperationResult {
                success: false,
                message: "Friendship not found".to_string(),
                friendship: None,
            }),
            Err(e) => Ok(FriendshipOperationResult {
                success: false,
                message: e.to_string(),
                friendship: None,
            }),
        }
    }

    // Update user
    async fn update_user(&self, ctx: &Context<'_>, id: ID, input: UpdateUserInput) -> Result<UserOperationResult> {
        let repo = ctx.data::<Repository>()?;
        let user_id = Uuid::parse_str(&id).map_err(|e| Error::new(e.to_string()))?;
        
        // For now, we'll just return the existing user since we don't have update methods
        // In a real implementation, you'd add update methods to the repository
        let user = repo.get_user_by_id(user_id).await
            .map_err(|e| Error::new(e.to_string()))?;
        
        match user {
            Some(user) => Ok(UserOperationResult {
                success: true,
                message: "User update functionality not implemented yet".to_string(),
                user: Some(user.into()),
            }),
            None => Ok(UserOperationResult {
                success: false,
                message: "User not found".to_string(),
                user: None,
            }),
        }
    }

    // Delete user
    async fn delete_user(&self, ctx: &Context<'_>, id: ID) -> Result<UserOperationResult> {
        let repo = ctx.data::<Repository>()?;
        let user_id = Uuid::parse_str(&id).map_err(|e| Error::new(e.to_string()))?;
        
        // For now, we'll just check if user exists since we don't have delete methods
        // In a real implementation, you'd add delete methods to the repository
        let user_exists = repo.get_user_by_id(user_id).await
            .map_err(|e| Error::new(e.to_string()))?
            .is_some();
        
        if user_exists {
            Ok(UserOperationResult {
                success: true,
                message: "User delete functionality not implemented yet".to_string(),
                user: None,
            })
        } else {
            Ok(UserOperationResult {
                success: false,
                message: "User not found".to_string(),
                user: None,
            })
        }
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;