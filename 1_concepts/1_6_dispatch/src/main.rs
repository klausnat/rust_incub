// TASK:
// Given the following Storage abstraction and User entity:

// trait Storage<K, V> {
//     fn set(&mut self, key: K, val: V);
//     fn get(&self, key: &K) -> Option<&V>;
//     fn remove(&mut self, key: &K) -> Option<V>;
// }

// struct User {
//     id: u64,
//     email: Cow<'static, str>,
//     activated: bool,
// }

use std::{borrow::Cow, collections::HashMap};

// Implement UserRepository type with injectable Storage implementation,
// which can get, add, update and remove User in the injected Storage.
// Make two different implementations: one should use dynamic dispatch for Storage injecting,
// and the other one should use static dispatch.
use step_1_6::*;
fn main() {
    let user1 = User {
        id: 1,
        email: Cow::Borrowed("web@gmail.com"),
        activated: true,
    };

    let user2 = User {
        id: 2,
        email: Cow::Borrowed("databada@gmail.com"),
        activated: false,
    };

    let user3 = User {
        id: 3,
        email: Cow::Borrowed("boboobo@gmail.com"),
        activated: true,
    };

    let user4 = User {
        id: 4,
        email: Cow::Borrowed("alabamala@gmail.com"),
        activated: true,
    };

    #[derive(Default)]
    struct HashMapStorage {
        data: HashMap<u64, User>,
    }

    // Dynamic
    impl Storage<u64, User> for HashMapStorage {
        fn set(&mut self, key: u64, val: User) {
            self.data.insert(key, val);
        }

        fn get(&self, key: &u64) -> Option<&User> {
            self.data.get(key)
        }

        fn remove(&mut self, key: &u64) -> Option<User> {
            self.data.remove(key)
        }
    }

    let storage = Box::new(HashMapStorage::default());
    let mut user_rep_dynamic: UserRepository = UserRepository::new(storage);

    user_rep_dynamic.add_user(user1);
    user_rep_dynamic.add_user(user2);

    println!("Dynamic rep: {:?}", user_rep_dynamic.get_user(&2));

    // Static

    let storage_static = HashMapStorage::default();
    let mut user_rep_static: UserRepositoryStatic<HashMapStorage> =
        UserRepositoryStatic::new(storage_static);

    user_rep_static.add_user(user3);
    user_rep_static.add_user(user4);

    println!("Static rep: {:?}", user_rep_static.get_user(&3));
}
