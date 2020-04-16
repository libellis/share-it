#[macro_use]
extern crate serde_derive;

pub mod soundcloud_api;
pub use soundcloud_api::*;

pub mod repositories;
pub use repositories::*;

pub mod user;
pub use user::*;

pub mod song;
pub use song::*;

pub mod playlist;
pub use playlist::*;

pub mod waitlist;
pub use waitlist::*;

pub mod test_tools;
pub use test_tools::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
