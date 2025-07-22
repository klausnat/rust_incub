use std::borrow::Cow;

// Storage: trait for common behavior
pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

// Convert UserRepository to a trait
trait UserRepository {
    fn add_user(&mut self, user: User) -> Result<(), UserError>;
    fn get_user(&self, id: u64) -> Option<&User>;
    fn remove_user(&mut self, id: u64);
}

pub struct UserRepositoryImpl {
    pub storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryImpl {
    pub fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        UserRepositoryImpl { storage }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn add_user(&mut self, user: User) -> Result<(), UserError> {
        if self.storage.get(&user.id).is_some() {
            return Err(UserError::AlreadyExists);
        }
        self.storage.set(user.id, user);
        Ok(())
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn remove_user(&mut self, id: u64) {
        self.storage.remove(&id);
    }
}

/* ================ TASK ========================= */
// create User command
pub struct CreateUser {
    pub user: User,
}
pub trait Command {
    fn command_name(&self) -> &str;
}

impl Command for CreateUser {
    fn command_name(&self) -> &str {
        "CreateUser"
    }
}

#[derive(Debug)]
pub enum UserError {
    AlreadyExists,
    RepositoryError,
}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;

    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        // Here we operate with the `UserRepository`
        // via its trait object `&dyn UserRepository`
        if ctx.get_user(cmd.user.id).is_some() {
            return Err(UserError::AlreadyExists);
        }
        ctx.add_user(cmd.user.clone())
    }
}

// Mock implementation and tests
struct MockUserRepository {
    users: Vec<User>,
}

impl UserRepository for MockUserRepository {
    fn add_user(&mut self, user: User) -> Result<(), UserError> {
        if self.users.iter().any(|u| u.id == user.id) {
            Err(UserError::AlreadyExists)
        } else {
            self.users.push(user);
            Ok(())
        }
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    fn remove_user(&mut self, id: u64) {
        self.users.retain(|user| user.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_create_user_success() {
        let user = User {
            id: 1,
            email: Cow::Borrowed("hello@gmail.com"),
            activated: false,
        };

        let cmd = CreateUser { user: user.clone() };

        let mut mockRepo = MockUserRepository { users: Vec::new() };
    }
}
