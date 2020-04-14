#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate domain_derive;

pub mod soundcloud_api;
pub use soundcloud_api::*;

pub mod collection_abstractions;
pub use collection_abstractions::*;

pub mod user;
pub use user::*;

pub mod song;
pub use song::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
