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

