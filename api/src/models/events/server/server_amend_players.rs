use crate::Event;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
  pub uuid: String,
  pub players: u32,
}

impl Event for Inner {
    fn handle(&self) {
        println!("ServerAmendPlayers: {self:?}");
    }
}