#[macro_use]
extern crate serde_derive;

pub mod user;
pub use user::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
