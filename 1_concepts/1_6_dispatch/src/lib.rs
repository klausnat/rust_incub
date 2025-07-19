use std::borrow::Cow;

// Storage: trait for common behavior
pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

// Dynamic dispatch
pub struct UserRepository {
    pub storage: Box<dyn Storage<u64, User>>,
}

impl UserRepository {
    pub fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        UserRepository { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user)
    }

    pub fn get_user(&self, id: &u64) -> Option<&User> {
        self.storage.get(id)
    }

    pub fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn remove_user(&mut self, id: &u64) {
        self.storage.remove(id);
    }
}

// Static dispatch
pub struct UserRepositoryStatic<T: Storage<u64, User>> {
    pub storage: T,
}

impl<T: Storage<u64, User>> UserRepositoryStatic<T> {
    pub fn new(storage: T) -> Self {
        UserRepositoryStatic { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user)
    }

    pub fn get_user(&self, id: &u64) -> Option<&User> {
        self.storage.get(id)
    }

    pub fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn remove_user(&mut self, id: &u64) {
        self.storage.remove(id);
    }
}
