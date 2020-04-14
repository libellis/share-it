use std::collections::HashMap;
use share_it::{User, UserRepository};
use crate::common::MockError;

// for naive testing
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
        let key = user.id();

        let result = if self.contains(key).unwrap() {
            None
        } else {
            self.data.insert(key, user.clone());
            Some(key)
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
        let key = user.id();

        let result = if self.contains(key).unwrap() {
            self.data.insert(key, user.clone());
            Some(key)
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
