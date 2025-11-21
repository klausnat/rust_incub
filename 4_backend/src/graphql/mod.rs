use async_graphql::*;
use chrono::{DateTime, Utc};

use crate::models::{User, Friendship, UserWithFriends, AuthUser};

// Custom scalar for DateTime<Utc> with Clone implementation
#[derive(Debug, Clone)]
pub struct DateTimeScalar(pub DateTime<Utc>);

#[Scalar]
impl ScalarType for DateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            let dt = value.parse::<DateTime<Utc>>()
                .map_err(|_| InputValueError::expected_type(value.into()))?;
            Ok(DateTimeScalar(dt))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for DateTimeScalar {
    fn from(dt: DateTime<Utc>) -> Self {
        DateTimeScalar(dt)
    }
}

#[derive(SimpleObject, Clone)]
pub struct UserObject {
    pub id: ID,
    pub name: String,
    pub created_at: Option<DateTimeScalar>,
    pub updated_at: Option<DateTimeScalar>,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        Self {
            id: ID::from(user.id),
            name: user.name,
            created_at: Some(DateTimeScalar(user.created_at)),
            updated_at: Some(DateTimeScalar(user.updated_at)),
        }
    }
}

impl From<AuthUser> for UserObject {
    fn from(auth_user: AuthUser) -> Self {
        Self {
            id: ID::from(auth_user.id),
            name: auth_user.name,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct FriendshipObject {
    pub id: ID,
    pub user_id: ID,
    pub friend_id: ID,
    pub created_at: DateTimeScalar,
}

impl From<Friendship> for FriendshipObject {
    fn from(friendship: Friendship) -> Self {
        Self {
            id: ID::from(friendship.id),
            user_id: ID::from(friendship.user_id),
            friend_id: ID::from(friendship.friend_id),
            created_at: DateTimeScalar(friendship.created_at),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct UserWithFriendsObject {
    pub id: ID,
    pub name: String,
    pub created_at: DateTimeScalar,
    pub updated_at: DateTimeScalar,
    pub friends: Option<Vec<UserObject>>,
}

impl From<UserWithFriends> for UserWithFriendsObject {
    fn from(user_with_friends: UserWithFriends) -> Self {
        Self {
            id: ID::from(user_with_friends.id),
            name: user_with_friends.name,
            created_at: DateTimeScalar(user_with_friends.created_at),
            updated_at: DateTimeScalar(user_with_friends.updated_at),
            friends: user_with_friends.friends.map(|friends| {
                friends.into_iter().map(UserObject::from).collect()
            }),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct FriendshipWithDetailsObject {
    pub id: ID,
    pub user_id: ID,
    pub friend_id: ID,
    pub friend_name: String,
    pub created_at: DateTimeScalar,
}

// Input Objects
#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub name: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct AddFriendInput {
    pub friend_name: String,
}

#[derive(InputObject)]
pub struct RemoveFriendInput {
    pub friendship_id: ID,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub password: Option<String>,
}

// Response Objects
#[derive(SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: UserObject,
}

#[derive(SimpleObject)]
pub struct FriendshipOperationResult {
    pub success: bool,
    pub message: String,
    pub friendship: Option<FriendshipObject>,
}

#[derive(SimpleObject)]
pub struct UserOperationResult {
    pub success: bool,
    pub message: String,
    pub user: Option<UserObject>,
}