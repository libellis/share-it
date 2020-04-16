use crate::user::User;

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
    use crate::MockUserRepository;
    use crate::repositories::abstractions::UserRepository;
    use crate::test_tools::factories::{new_test_user, new_test_playlist};

    #[test]
    #[allow(unused)]
    fn test_add_user() {
        let user_id = 0;
        let test_user = new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        user_repo.insert(&test_user);
        let success_result = user_repo.get(user_id).unwrap();

        assert_eq!(&success_result.unwrap(), &test_user)
    }

    #[test]
    #[allow(unused)]
    fn test_cant_add_duplicate() {
        let user_id = 0;
        let test_user = new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        let returned_user = user_repo.insert(&test_user).unwrap();
        assert!(returned_user.is_some());

        let success_result = user_repo.get(user_id).unwrap();
        assert_eq!(&success_result.unwrap(), &test_user);

        let failure_result = user_repo.insert(&test_user).unwrap();
        assert!(failure_result.is_none());
    }

    #[test]
    #[allow(unused)]
    fn test_update_user() {
        let user_id = 0;
        let mut test_user = new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        let mut returned_user = user_repo.insert(&test_user).unwrap();
        assert!(returned_user.is_some());

        let original_user = test_user.clone();

        let new_playlist = new_test_playlist(user_id, 0);
        test_user.add_playlist(new_playlist);
        user_repo.update(&test_user).unwrap();

        let updated_user = user_repo.get(user_id).unwrap();
        assert_ne!(Some(original_user), updated_user)
    }

    #[test]
    #[allow(unused)]
    fn test_remove_user() {
        let user_id = 0;
        let test_user = new_test_user(user_id);
        let mut user_repo = MockUserRepository::new();
        user_repo.insert(&test_user);

        // we first check that user is in repo
        assert!(user_repo.contains(user_id).unwrap());

        user_repo.remove(user_id);
        assert!(!user_repo.contains(user_id).unwrap())
    }
}
