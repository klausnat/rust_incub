use im::{HashMap, Vector};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct UserId {
    id: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct User {
    id: UserId,
    nickname: String,
}

trait UserRepository {
    // returns single user by it's id
    fn get_user_by_id(&self, id: UserId) -> Result<User, RepositoryError>;
    fn get_users(&self, ids: Vec<UserId>) -> Result<Vec<&User>, RepositoryError>;
    fn get_by_nick(&self, piece: &str) -> Result<Vec<UserId>, RepositoryError>;
}

struct MyUsersVector {
    users: Vector<User>,
}

#[derive(Debug, PartialEq, Eq)]
enum RepositoryError {
    No_User_With_Such_Id,
    No_Users_With_Such_Ids,
    No_Users_Containing_Piece_Of_Nickname,
}

impl UserRepository for MyUsersVector {
    fn get_user_by_id(&self, id: UserId) -> Result<User, RepositoryError> {
        let res = self
            .users
            .iter()
            .find(|x| x.id == id)
            .ok_or(RepositoryError::No_User_With_Such_Id);

        res.cloned()
    }

    fn get_users(&self, ids: Vec<UserId>) -> Result<Vec<&User>, RepositoryError> {
        let res: Vec<&User> = self.users.iter().filter(|x| ids.contains(&x.id)).collect();
        if res.is_empty() {
            Err(RepositoryError::No_Users_With_Such_Ids)
        } else {
            Ok(res)
        }
    }

    fn get_by_nick(&self, piece: &str) -> Result<Vec<UserId>, RepositoryError> {
        let res: Vec<UserId> = self
            .users
            .iter()
            .filter(|x| x.nickname.contains(piece))
            .map(|x| x.id)
            .collect();
        if res.is_empty() {
            Err(RepositoryError::No_Users_Containing_Piece_Of_Nickname)
        } else {
            Ok(res)
        }
    }
}

struct MyUsersHashMap {
    users: HashMap<UserId, User>,
}

impl UserRepository for MyUsersHashMap {
    fn get_user_by_id(&self, id: UserId) -> Result<User, RepositoryError> {
        todo!()
    }

    fn get_users(&self, ids: Vec<UserId>) -> Result<Vec<&User>, RepositoryError> {
        todo!()
    }

    fn get_by_nick(&self, piece: &str) -> Result<Vec<UserId>, RepositoryError> {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_by_user_id_Vector() {
        let user1 = User {
            id: UserId { id: 3 },
            nickname: String::from("somenickname"),
        };
        let user2 = User {
            id: UserId { id: 4 },
            nickname: String::from("nicksomenickname"),
        };

        let cloned_user2 = user2.clone();

        let user3 = User {
            id: UserId { id: 7 },
            nickname: String::from("ssssname"),
        };

        let user_repository = MyUsersVector {
            users: Vector::from(vec![user1, user2, user3]),
        };
        let id = UserId { id: 4 };
        let result = user_repository.get_user_by_id(id).unwrap();
        assert_eq!(result, cloned_user2);
    }

    #[test]
    fn test_get_users_ids_Vector() {
        let user1 = User {
            id: UserId { id: 3 },
            nickname: String::from("somenickname"),
        };

        let user1_h = &user1;

        let user2 = User {
            id: UserId { id: 4 },
            nickname: String::from("nicksomenickname"),
        };

        let user3 = User {
            id: UserId { id: 7 },
            nickname: String::from("ssssname"),
        };

        let user3_h = &user3;

        let user_repository = MyUsersVector {
            users: Vector::from(vec![user1.clone(), user2, user3.clone()]),
        };
        let ids = vec![UserId { id: 3 }, UserId { id: 7 }, UserId { id: 9 }];
        let result = user_repository.get_users(ids).unwrap();

        assert_eq!(result, vec![user1_h, user3_h]);
    }

    #[test]
    fn test_get_by_nick_Vector() {
        let user1 = User {
            id: UserId { id: 3 },
            nickname: String::from("somenickname"),
        };
        let user2 = User {
            id: UserId { id: 4 },
            nickname: String::from("nicksomenickname"),
        };

        let user3 = User {
            id: UserId { id: 7 },
            nickname: String::from("ssssname"),
        };

        let user_repository = MyUsersVector {
            users: Vector::from(vec![user1, user2, user3]),
        };
        
        let result = user_repository.get_by_nick("nick").unwrap();
        assert_eq!(result, vec![UserId{id:3}, UserId{id: 4}]);

        let result = user_repository.get_by_nick("bbbbb");
        assert_eq!(result, Err(RepositoryError::No_Users_Containing_Piece_Of_Nickname));
    }

    #[test]
    fn test_get_by_user_id_HashMap() {
        // let result = user_repository.get_by_nick("hello");
        // assert_eq!(result, 4);
    }
}
