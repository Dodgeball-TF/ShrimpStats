use crate::Event;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
  pub uuid: String,
  pub rcon_password: String,
}

impl Event for Inner {
    fn handle(&self) {
        println!("ServerAmendRconPassword: {self:?}");
    }
}