// @generated automatically by Diesel CLI.

diesel::table! {
    roles (slug) {
        slug -> Varchar,
        name -> Varchar,
        permissions -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (user_id, role_slug) {
        user_id -> Int4,
        role_slug -> Varchar,
        assigned_at -> Timestamp,
    }
}

diesel::joinable!(users_roles -> roles (role_slug));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(roles, users, users_roles,);
