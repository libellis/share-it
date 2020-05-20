#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

pub mod soundcloud_api;
pub use soundcloud_api::*;

pub mod repositories;
pub use repositories::*;

pub mod user;

pub mod song;
pub use song::*;

pub mod chatroom;
pub mod waitlist;
pub mod playlist;

pub mod test_tools;
pub use test_tools::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
