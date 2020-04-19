use std::fmt;
use std::error;
use std::collections::HashMap;
use crate::user::User;
use crate::repositories::abstractions::Repository;
use rusty_ulid::Ulid;
use crate::waitlist::Waitlist;

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

#[derive(Clone)]
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

impl Repository<u32, User> for MockUserRepository {
    // For ease of use in testing. Use real error type in production.
    type Error = MockError;

    fn insert(&mut self, entity: &User) -> Result<Option<u32>, Self::Error> {
        let result = if self.contains(&entity.id()).unwrap() {
            None
        } else {
            self.data.insert(entity.id(), entity.clone());
            Some(entity.id())
        };

        Ok(result)
    }

    fn get(&mut self, key: &u32) -> Result<Option<User>, Self::Error> {
        let result = if let Some(user) = self.data.get(key) {
            Some(user.clone())
        } else {
            None
        };

        Ok(result)
    }

    fn update(&mut self, entity: &User) -> Result<Option<u32>, Self::Error> {
        let result = if self.contains(&entity.id()).unwrap() {
            self.data.insert(entity.id(), entity.clone());
            Some(entity.id())
        } else {
            None
        };

        Ok(result)
    }

    fn remove(&mut self, key: &u32) -> Result<Option<u32>, Self::Error> {
        let result = self.data.remove(key);
        if let Some(user) = result {
            Ok(Some(user.id()))
        } else {
            Ok(None)
        }
    }
}

impl Repository<u32, User> for &mut MockUserRepository {
    // For ease of use in testing. Use real error type in production.
    type Error = MockError;

    fn insert(&mut self, entity: &User) -> Result<Option<u32>, Self::Error> {
        let result = if self.contains(&entity.id()).unwrap() {
            None
        } else {
            self.data.insert(entity.id(), entity.clone());
            Some(entity.id())
        };

        Ok(result)
    }

    fn get(&mut self, key: &u32) -> Result<Option<User>, Self::Error> {
        let result = if let Some(user) = self.data.get(&key) {
            Some(user.clone())
        } else {
            None
        };

        Ok(result)
    }

    fn update(&mut self, entity: &User) -> Result<Option<u32>, Self::Error> {
        let result = if self.contains(&entity.id()).unwrap() {
            self.data.insert(entity.id(), entity.clone());
            Some(entity.id())
        } else {
            None
        };

        Ok(result)
    }

    fn remove(&mut self, key: &u32) -> Result<Option<u32>, Self::Error> {
        let result = self.data.remove(&key);
        if let Some(user) = result {
            Ok(Some(user.id()))
        } else {
            Ok(None)
        }
    }
}
