use crate::User;

// TODO: If we end up needing more than one repository type, switch this to a generic Repository<T>
pub(crate) trait UserRepository {
    /// An error that communicates that something went wrong when communicating with the external api, database etc.
    type Error: std::error::Error + std::fmt::Display + 'static + Send;

    /// Inserts a User into the underlying persistent storage (MySQL, Postgres, Mongo etc.).
    ///
    /// If the underlying storage did not have the User present, then insert is successful and the primary key is returned.
    /// This allows for auto-generated ids to be returned after insert.
    ///
    /// If the underlying storage does have the key present, then [`None`] is returned.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn insert(&mut self, user: &User) -> Result<Option<u32>, Self::Error>;

    /// Returns the User with the supplied user_id as an owned type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get(&mut self, user_id: u32) -> Result<Option<User>, Self::Error>;

    /// Returns `true` if the underlying storage contains an entity at the specified key,
    /// and otherwise returns `false`.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn contains(&mut self, user_id: u32) -> Result<bool, Self::Error> {
        Ok(self.get(user_id)?.is_some())
    }

    /// Updates the User in the underlying storage mechanism and if successful returns the primary
    /// key to the caller. If the User does not exist in the database (it's unique
    /// id is not in use), then we return [`None`].
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn update(&mut self, user: &User) -> Result<Option<u32>, Self::Error>;

    /// Removes a User from the underlying storage at the given user_id,
    /// returning the user_id if the user was in the database and deleted, and otherwise returning [`None`]
    /// if the entity was not found (no rows effected by the operation).
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn remove(&mut self, user_id: u32) -> Result<Option<u32>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::error;
    use std::collections::HashMap;

    use crate::{User, UserRepository};

    #[derive(Debug, Clone)]
    pub struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "this is a mock error")
        }
    }

    impl error::Error for MockError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            // Generic error, underlying cause isn't tracked.
            None
        }
    }

    pub struct MockUserRepository {
        data: HashMap<u32, User>
    }

    impl MockUserRepository {
        pub fn new() -> MockUserRepository {
            MockUserRepository {
                data: HashMap::new(),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        // For ease of use in testing. Use real error type in production.
        type Error = MockError;

        fn insert(&mut self, user: &User) -> Result<Option<u32>, Self::Error> {
            let result = if self.contains(user.id()).unwrap() {
                None
            } else {
                self.data.insert(user.id(), user.clone());
                Some(user.id())
            };

            Ok(result)
        }

        fn get(&mut self, user_id: u32) -> Result<Option<User>, Self::Error> {
            let result = if let Some(user) = self.data.get(&user_id) {
                Some(user.clone())
            } else {
                None
            };

            Ok(result)
        }

        fn update(&mut self, user: &User) -> Result<Option<u32>, Self::Error> {
            let result = if self.contains(user.id()).unwrap() {
                self.data.insert(user.id(), user.clone());
                Some(user.id())
            } else {
                None
            };

            Ok(result)
        }

        fn remove(&mut self, user_id: u32) -> Result<Option<u32>, Self::Error> {
            let result = self.data.remove(&user_id);
            if let Some(user) = result {
                Ok(Some(user.id()))
            } else {
                Ok(None)
            }
        }
    }

    #[test]
    #[allow(unused)]
    fn test_add_user() {
        let user_id = 0;
        let test_user = User::new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        user_repo.insert(&test_user);
        let success_result = user_repo.get(user_id).unwrap();

        assert_eq!(&success_result.unwrap(), &test_user)
    }

    #[test]
    #[allow(unused)]
    fn test_cant_add_duplicate() {
        let user_id = 0;
        let test_user = User::new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        let returned_entity = user_repo.insert(&test_user).unwrap();
        assert!(returned_entity.is_some());

        let success_result = user_repo.get(user_id).unwrap();
        assert_eq!(&success_result.unwrap(), &test_user);

        let failure_result = user_repo.insert(&test_user).unwrap();
        assert!(failure_result.is_none());
    }

// TODO: Test this once we add playlist support. Check that we can update a users playlists and it persists.
// #[test]
// #[allow(unused)]
// fn test_update_user() {
//     let user_id = 0;
//     let mut test_user = common::create_test_user(user_id);
//     let mut user_repo = MockUserRepository::new();
//     let returned_entity = user_repo.insert(&test_user).unwrap();
//     assert!(returned_entity.is_some());
//
//     // TODO: Update users playlists here.
//     user_repo.update(&test_user).unwrap();
//     // check that we get back Some() which implies updating worked.
//     assert!(returned_entity.is_some());
//
//     let updated_user = user_repo.get(user_id).unwrap();
//     assert_eq!(updated_user.unwrap().first_name(), &updated_name);
// }

    #[test]
    #[allow(unused)]
    fn test_remove_user() {
        let user_id = 0;
        let test_user = User::new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        user_repo.insert(&test_user);

        // we first check that user is in repo
        assert!(user_repo.contains(user_id).unwrap());

        user_repo.remove(user_id);
        assert!(!user_repo.contains(user_id).unwrap())
    }
}
