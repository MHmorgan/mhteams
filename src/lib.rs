//!
//!  MHteams provides an easy and idiomatic way of creating and sending messages to MS Teams webhooks.
//! 

extern crate serde;
extern crate serde_json;

mod message;
pub use message::Message;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
