#[macro_use]
extern crate serde_derive;

pub mod soundcloud_api;
pub use soundcloud_api::*;

pub mod user;
pub use user::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
