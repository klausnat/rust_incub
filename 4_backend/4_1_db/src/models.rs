use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::roles)]
pub struct Role {
    pub slug: String,
    pub name: String,
    pub permissions: Option<String>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub slug: String,
    pub name: String,
    pub permissions: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roles)]
pub struct UpdateRole {
    pub name: Option<String>,
    pub permissions: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Identifiable, Associations)]
#[diesel(table_name = crate::schema::users_roles)]
#[diesel(belongs_to(User))]
#[diesel(primary_key(user_id, role_slug))]
pub struct UserRole {
    pub user_id: i32,
    pub role_slug: String,
    pub assigned_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_slug: String,
}
